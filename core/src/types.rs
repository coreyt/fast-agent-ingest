/// Options controlling HTML → Markdown conversion.
#[derive(Debug, Clone)]
pub struct ConversionOptions {
    /// Run readability-style main-content extraction before converting.
    /// When false the entire document body is converted as-is.
    pub extract_main_content: bool,

    /// Emit Markdown image syntax `![alt](src)` for `<img>` elements.
    pub include_images: bool,

    /// Emit Markdown link syntax `[text](href)` for `<a>` elements.
    pub include_links: bool,

    /// Fence style used for code blocks.
    pub code_fence_style: CodeFenceStyle,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            extract_main_content: true,
            include_images: true,
            include_links: true,
            code_fence_style: CodeFenceStyle::Backtick,
        }
    }
}

/// How to render fenced code blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeFenceStyle {
    /// ``` (default, most LLM-friendly)
    Backtick,
    /// ~~~ (alternative)
    Tilde,
}

/// Output of a conversion operation.
#[derive(Debug, Default)]
pub struct ConversionResult {
    /// The converted Markdown text.
    pub markdown: String,

    /// Page title extracted from `<title>` or the first `<h1>`, if found.
    pub title: Option<String>,

    /// Meta description content, if present.
    pub description: Option<String>,
}
