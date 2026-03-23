use markup5ever_rcdom::{Handle, NodeData};
use html5ever::local_name;

use crate::types::ConversionOptions;

/// Render an inline element (and its children) to a Markdown string.
pub fn render_inline(node: &Handle, opts: &ConversionOptions) -> String {
    match &node.data {
        NodeData::Text { contents } => {
            // Normalise whitespace: collapse runs of whitespace to a single space.
            let raw = contents.borrow();
            normalise_whitespace(raw.as_ref())
        }

        NodeData::Element { name, attrs, .. } => {
            let tag = name.local.as_ref();
            let children: String = node.children.borrow().iter()
                .map(|c| render_inline(c, opts))
                .collect();

            match tag {
                "strong" | "b" => format!("**{}**", children.trim()),
                "em"    | "i" => format!("*{}*", children.trim()),
                "s" | "del" | "strike" => format!("~~{}~~", children.trim()),
                "code"         => format!("`{}`", children),
                "mark"         => children, // no Markdown equivalent — pass through

                "a" if opts.include_links => {
                    let attrs = attrs.borrow();
                    let href = attrs.iter()
                        .find(|a| a.name.local == local_name!("href"))
                        .map(|a| a.value.as_ref())
                        .unwrap_or("");
                    if href.is_empty() || href.starts_with("javascript:") {
                        children
                    } else {
                        format!("[{}]({})", children.trim(), href)
                    }
                }
                "a" => children,

                "img" if opts.include_images => {
                    let attrs = attrs.borrow();
                    let src = attrs.iter()
                        .find(|a| a.name.local == local_name!("src"))
                        .map(|a| a.value.as_ref())
                        .unwrap_or("");
                    let alt = attrs.iter()
                        .find(|a| a.name.local == local_name!("alt"))
                        .map(|a| a.value.as_ref())
                        .unwrap_or("");
                    if src.is_empty() { String::new() } else { format!("![{}]({})", alt, src) }
                }
                "img" => String::new(),

                "br"  => "\n".to_owned(),
                "wbr" => String::new(),

                // Anything else: recurse into children transparently.
                _ => children,
            }
        }

        // Comments, doctypes, etc. — drop.
        _ => String::new(),
    }
}

fn normalise_whitespace(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_was_space = false;
    for c in s.chars() {
        if c.is_whitespace() {
            if !last_was_space {
                out.push(' ');
            }
            last_was_space = true;
        } else {
            out.push(c);
            last_was_space = false;
        }
    }
    out
}
