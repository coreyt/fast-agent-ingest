use markup5ever_rcdom::{Handle, NodeData};
use html5ever::local_name;

use crate::types::{ConversionOptions, CodeFenceStyle};
use super::inline::render_inline;

/// Render a block-level element to Markdown, recursing into children.
pub fn render_block(node: &Handle, opts: &ConversionOptions, depth: usize) -> String {
    match &node.data {
        NodeData::Text { contents } => {
            let text = contents.borrow();
            let trimmed = text.trim();
            if trimmed.is_empty() { String::new() } else { format!("{}\n", trimmed) }
        }

        NodeData::Element { name, attrs, .. } => {
            let tag = name.local.as_ref();

            // Skip noise tags entirely.
            if crate::extractor::noise::is_noise_tag(tag) {
                return String::new();
            }

            // Skip noise class/id elements.
            {
                let attrs_ref = attrs.borrow();
                let is_noise = attrs_ref.iter().any(|a| {
                    (a.name.local == local_name!("class") || a.name.local == local_name!("id"))
                        && crate::extractor::noise::is_noise_attr(&a.value)
                });
                if is_noise {
                    return String::new();
                }
            }

            match tag {
                "h1" => heading(node, opts, 1),
                "h2" => heading(node, opts, 2),
                "h3" => heading(node, opts, 3),
                "h4" => heading(node, opts, 4),
                "h5" => heading(node, opts, 5),
                "h6" => heading(node, opts, 6),

                "p"  => paragraph(node, opts),
                "br" => "\n".to_owned(),
                "hr" => "\n---\n\n".to_owned(),

                "blockquote" => blockquote(node, opts),

                "pre" => code_block(node, opts),
                // Bare <code> at block level — treat as code block.
                "code" if is_block_code(node) => code_block(node, opts),

                "ul" => list(node, opts, false, depth),
                "ol" => list(node, opts, true, depth),

                "table" => table(node, opts),

                "figure" => figure(node, opts),

                // Generic containers — just recurse.
                "div" | "article" | "section" | "main" | "body"
                | "span" | "details" | "summary" | "figcaption" => {
                    children_to_markdown(node, opts, depth)
                }

                // Inline elements appearing at block level.
                _ => {
                    let inline = render_inline(node, opts);
                    if inline.trim().is_empty() {
                        String::new()
                    } else {
                        format!("{}\n\n", inline.trim())
                    }
                }
            }
        }

        NodeData::Document => children_to_markdown(node, opts, depth),
        _ => String::new(),
    }
}

// ── Block renderers ───────────────────────────────────────────────────────────

fn heading(node: &Handle, opts: &ConversionOptions, level: usize) -> String {
    let prefix = "#".repeat(level);
    let text: String = node.children.borrow().iter()
        .map(|c| render_inline(c, opts))
        .collect();
    format!("{} {}\n\n", prefix, text.trim())
}

fn paragraph(node: &Handle, opts: &ConversionOptions) -> String {
    let text: String = node.children.borrow().iter()
        .map(|c| render_inline(c, opts))
        .collect();
    let trimmed = text.trim();
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("{}\n\n", trimmed)
    }
}

fn blockquote(node: &Handle, opts: &ConversionOptions) -> String {
    let inner = children_to_markdown(node, opts, 0);
    inner.lines()
        .map(|l| format!("> {}", l))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n\n"
}

fn code_block(node: &Handle, opts: &ConversionOptions) -> String {
    let fence = match opts.code_fence_style {
        CodeFenceStyle::Backtick => "```",
        CodeFenceStyle::Tilde    => "~~~",
    };

    // Try to detect language from class="language-xxx" or class="lang-xxx".
    let lang = if let NodeData::Element { attrs, .. } = &node.data {
        attrs.borrow().iter()
            .find(|a| a.name.local == local_name!("class"))
            .and_then(|a| {
                a.value.split_whitespace()
                    .find(|c| c.starts_with("language-") || c.starts_with("lang-"))
                    .map(|c| c.split_once('-').map(|(_, l)| l).unwrap_or("").to_owned())
            })
            .unwrap_or_default()
    } else {
        String::new()
    };

    let code = collect_text_raw(node);
    format!("{}{}\n{}\n{}\n\n", fence, lang, code.trim_end(), fence)
}

fn list(node: &Handle, opts: &ConversionOptions, ordered: bool, depth: usize) -> String {
    let indent = "  ".repeat(depth);
    let mut out = String::new();
    let mut index = 1usize;

    for child in node.children.borrow().iter() {
        if let NodeData::Element { name, .. } = &child.data {
            if name.local == local_name!("li") {
                let marker = if ordered {
                    format!("{}{}. ", indent, index)
                } else {
                    format!("{}- ", indent)
                };
                index += 1;

                // Render li contents — nested lists get deeper depth.
                let inner = li_content(child, opts, depth + 1);
                out.push_str(&marker);
                out.push_str(inner.trim_start());
                if !out.ends_with('\n') {
                    out.push('\n');
                }
            }
        }
    }
    out.push('\n');
    out
}

fn li_content(node: &Handle, opts: &ConversionOptions, depth: usize) -> String {
    let mut out = String::new();
    for child in node.children.borrow().iter() {
        match &child.data {
            NodeData::Element { name, .. }
                if name.local == local_name!("ul") || name.local == local_name!("ol") =>
            {
                let ordered = name.local == local_name!("ol");
                out.push('\n');
                out.push_str(&list(child, opts, ordered, depth));
            }
            _ => out.push_str(&render_inline(child, opts)),
        }
    }
    out
}

fn table(node: &Handle, opts: &ConversionOptions) -> String {
    // Collect rows from thead/tbody/tr.
    let mut rows: Vec<Vec<String>> = Vec::new();
    collect_rows(node, opts, &mut rows);

    if rows.is_empty() {
        return String::new();
    }

    let cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    if cols == 0 {
        return String::new();
    }

    // Determine column widths for alignment.
    let widths: Vec<usize> = (0..cols)
        .map(|c| rows.iter().map(|r| r.get(c).map(|s| s.len()).unwrap_or(0)).max().unwrap_or(0).max(3))
        .collect();

    let mut out = String::new();
    for (i, row) in rows.iter().enumerate() {
        out.push('|');
        for (j, w) in widths.iter().enumerate() {
            let cell = row.get(j).map(|s| s.as_str()).unwrap_or("");
            out.push_str(&format!(" {:width$} |", cell, width = w));
        }
        out.push('\n');

        // After the first row (header), emit separator.
        if i == 0 {
            out.push('|');
            for w in &widths {
                out.push_str(&format!(" {} |", "-".repeat(*w)));
            }
            out.push('\n');
        }
    }
    out.push('\n');
    out
}

fn collect_rows(node: &Handle, opts: &ConversionOptions, rows: &mut Vec<Vec<String>>) {
    match &node.data {
        NodeData::Element { name, .. } => {
            let tag = name.local.as_ref();
            if tag == "tr" {
                let cells: Vec<String> = node.children.borrow().iter()
                    .filter(|c| matches!(&c.data, NodeData::Element { name, .. } if
                        name.local == local_name!("td") || name.local == local_name!("th")))
                    .map(|c| {
                        let t: String = c.children.borrow().iter()
                            .map(|cc| render_inline(cc, opts))
                            .collect();
                        t.trim().to_owned()
                    })
                    .collect();
                if !cells.is_empty() {
                    rows.push(cells);
                }
                return; // Don't recurse further into tr
            }
            for child in node.children.borrow().iter() {
                collect_rows(child, opts, rows);
            }
        }
        _ => {
            for child in node.children.borrow().iter() {
                collect_rows(child, opts, rows);
            }
        }
    }
}

fn figure(node: &Handle, opts: &ConversionOptions) -> String {
    children_to_markdown(node, opts, 0)
}

// ── Utilities ────────────────────────────────────────────────────────────────

fn children_to_markdown(node: &Handle, opts: &ConversionOptions, depth: usize) -> String {
    node.children.borrow().iter()
        .map(|c| render_block(c, opts, depth))
        .collect()
}

fn collect_text_raw(node: &Handle) -> String {
    match &node.data {
        NodeData::Text { contents } => contents.borrow().to_string(),
        _ => node.children.borrow().iter()
            .map(collect_text_raw)
            .collect::<Vec<_>>()
            .join(""),
    }
}

fn is_block_code(node: &Handle) -> bool {
    // A <code> element is treated as a block code fence if its parent is <pre>
    // or if it has no surrounding inline siblings. Simple heuristic: if the
    // code text contains a newline, treat as block.
    collect_text_raw(node).contains('\n')
}
