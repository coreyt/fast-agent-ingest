pub mod noise;
pub mod scoring;

use markup5ever_rcdom::{Handle, NodeData};
use html5ever::local_name;

/// Extracted content ready for Markdown serialisation.
pub struct ExtractedContent {
    /// The root node of the content subtree to serialise.
    pub root: Handle,
    /// Page title, if found.
    pub title: Option<String>,
    /// Meta description, if found.
    pub description: Option<String>,
}

/// Walk the DOM and return the content subtree.
///
/// When `extract_main_content` is true the function scores all candidate
/// block-level nodes (Readability-style) and returns the highest-scoring one.
/// When false it returns the `<body>` node unchanged.
pub fn extract(dom: &markup5ever_rcdom::RcDom, extract_main: bool) -> ExtractedContent {
    let document = dom.document.clone();
    let title = find_title(&document);
    let description = find_meta_description(&document);

    let body = find_body(&document);

    let root = if extract_main {
        body.as_ref()
            .and_then(|b| score_and_select(b))
            .or_else(|| body.clone())
            .unwrap_or_else(|| document.clone())
    } else {
        body.unwrap_or_else(|| document.clone())
    };

    ExtractedContent { root, title, description }
}

// ── Private helpers ───────────────────────────────────────────────────────────

fn find_title(node: &Handle) -> Option<String> {
    match &node.data {
        NodeData::Element { name, .. } if name.local == local_name!("title") => {
            let text = collect_text(node);
            let trimmed = text.trim().to_owned();
            if trimmed.is_empty() { None } else { Some(trimmed) }
        }
        _ => {
            for child in node.children.borrow().iter() {
                if let Some(t) = find_title(child) {
                    return Some(t);
                }
            }
            None
        }
    }
}

fn find_meta_description(node: &Handle) -> Option<String> {
    match &node.data {
        NodeData::Element { name, attrs, .. }
            if name.local == local_name!("meta") =>
        {
            let attrs = attrs.borrow();
            let is_description = attrs.iter().any(|a| {
                a.name.local == local_name!("name")
                    && a.value.to_lowercase().contains("description")
            });
            if is_description {
                return attrs.iter()
                    .find(|a| a.name.local == local_name!("content"))
                    .map(|a| a.value.to_string());
            }
            None
        }
        _ => {
            for child in node.children.borrow().iter() {
                if let Some(d) = find_meta_description(child) {
                    return Some(d);
                }
            }
            None
        }
    }
}

fn find_body(node: &Handle) -> Option<Handle> {
    match &node.data {
        NodeData::Element { name, .. } if name.local == local_name!("body") => {
            Some(node.clone())
        }
        _ => {
            for child in node.children.borrow().iter() {
                if let Some(b) = find_body(child) {
                    return Some(b);
                }
            }
            None
        }
    }
}

/// Assign content scores to all candidate nodes and return the best one.
///
/// TODO: full Readability-style scoring with link-density penalty,
///       paragraph text analysis, and parent propagation.
fn score_and_select(body: &Handle) -> Option<Handle> {
    let mut best: Option<(Handle, f32)> = None;
    score_node(body, &mut best);
    best.map(|(h, _)| h)
}

fn score_node(node: &Handle, best: &mut Option<(Handle, f32)>) {
    if let NodeData::Element { name, attrs, .. } = &node.data {
        let tag = name.local.as_ref();

        // Skip noise elements entirely.
        if noise::is_noise_tag(tag) {
            return;
        }

        let attrs = attrs.borrow();
        let is_noise = attrs.iter().any(|a| {
            (a.name.local == local_name!("class") || a.name.local == local_name!("id"))
                && noise::is_noise_attr(&a.value)
        });
        if is_noise {
            return;
        }

        let mut score = scoring::initial_content_score(tag);
        let text = collect_text(node);
        // Add 1 point per comma (rough proxy for prose density).
        score += text.matches(',').count() as f32;
        // Add points proportional to text length, capped at 3.
        score += (text.len() as f32 / 100.0).min(3.0);

        if score > best.as_ref().map(|(_, s)| *s).unwrap_or(f32::NEG_INFINITY) {
            *best = Some((node.clone(), score));
        }
    }

    for child in node.children.borrow().iter() {
        score_node(child, best);
    }
}

fn collect_text(node: &Handle) -> String {
    match &node.data {
        NodeData::Text { contents } => contents.borrow().to_string(),
        _ => node.children.borrow().iter()
            .map(collect_text)
            .collect::<Vec<_>>()
            .join(""),
    }
}
