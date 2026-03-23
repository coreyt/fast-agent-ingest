pub mod block;
pub mod inline;

use crate::extractor::ExtractedContent;
use crate::types::{ConversionOptions, ConversionResult};

/// Serialise the extracted content tree to Markdown.
pub fn to_markdown(content: ExtractedContent, opts: &ConversionOptions) -> ConversionResult {
    let raw = block::render_block(&content.root, opts, 0);
    let markdown = tidy_markdown(&raw);
    ConversionResult {
        markdown,
        title: content.title,
        description: content.description,
    }
}

/// Post-process the raw Markdown output:
///   - Collapse runs of 3+ blank lines to 2.
///   - Trim leading/trailing whitespace.
fn tidy_markdown(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut blank_run = 0usize;

    for line in raw.lines() {
        if line.trim().is_empty() {
            blank_run += 1;
            if blank_run <= 2 {
                out.push('\n');
            }
        } else {
            blank_run = 0;
            out.push_str(line);
            out.push('\n');
        }
    }

    out.trim().to_owned() + "\n"
}
