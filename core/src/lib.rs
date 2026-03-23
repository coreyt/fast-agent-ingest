pub mod converter;
pub mod extractor;
pub mod parser;
pub mod types;

pub use types::{CodeFenceStyle, ConversionOptions, ConversionResult};

/// Convert an HTML string to Markdown.
///
/// # Example
/// ```rust
/// use fast_agent_ingest_core::{convert, ConversionOptions};
///
/// let html = "<html><body><h1>Hello</h1><p>World</p></body></html>";
/// let opts = ConversionOptions { extract_main_content: false, ..Default::default() };
/// let result = convert(html, &opts);
/// assert!(result.markdown.contains("# Hello"));
/// assert!(result.markdown.contains("World"));
/// ```
pub fn convert(html: &str, opts: &ConversionOptions) -> ConversionResult {
    let dom = parser::parse(html);
    let content = extractor::extract(&dom, opts.extract_main_content);
    converter::to_markdown(content, opts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn fixtures_dir() -> &'static Path {
        Path::new("../tests/fixtures")
    }

    #[test]
    fn no_extraction_body_pass_through() {
        let html = "<html><body><h1>Hello</h1><p>World</p></body></html>";
        let opts = ConversionOptions { extract_main_content: false, ..Default::default() };
        let r = convert(html, &opts);
        assert!(r.markdown.contains("# Hello"), "got: {}", r.markdown);
        assert!(r.markdown.contains("World"),   "got: {}", r.markdown);
    }

    #[test]
    fn heading_levels() {
        let html = "<html><body><h1>H1</h1><h2>H2</h2><h3>H3</h3></body></html>";
        let opts = ConversionOptions { extract_main_content: false, ..Default::default() };
        let r = convert(html, &opts);
        assert!(r.markdown.contains("# H1"));
        assert!(r.markdown.contains("## H2"));
        assert!(r.markdown.contains("### H3"));
    }

    #[test]
    fn noise_tags_stripped() {
        let html = "<html><body><script>alert(1)</script><p>Content</p><style>.x{}</style></body></html>";
        let opts = ConversionOptions { extract_main_content: false, ..Default::default() };
        let r = convert(html, &opts);
        assert!(!r.markdown.contains("alert"));
        assert!(!r.markdown.contains(".x"));
        assert!(r.markdown.contains("Content"));
    }

    #[test]
    fn inline_formatting() {
        let html = "<html><body><p><strong>bold</strong> and <em>italic</em></p></body></html>";
        let opts = ConversionOptions { extract_main_content: false, ..Default::default() };
        let r = convert(html, &opts);
        assert!(r.markdown.contains("**bold**"));
        assert!(r.markdown.contains("*italic*"));
    }

    #[test]
    fn links_and_images() {
        let html = r#"<html><body>
            <p><a href="https://example.com">Example</a></p>
            <img src="pic.jpg" alt="A picture">
        </body></html>"#;
        let opts = ConversionOptions { extract_main_content: false, ..Default::default() };
        let r = convert(html, &opts);
        assert!(r.markdown.contains("[Example](https://example.com)"));
        assert!(r.markdown.contains("![A picture](pic.jpg)"));
    }
}
