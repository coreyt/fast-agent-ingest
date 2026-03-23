#![deny(clippy::all)]

use napi_derive::napi;
use fast_agent_ingest_core::{convert as core_convert, ConversionOptions};

/// Options controlling HTML → Markdown conversion.
#[napi(object)]
pub struct JsConversionOptions {
    /// Run readability-style main-content extraction.
    pub extract_main_content: Option<bool>,
    /// Emit Markdown image syntax.
    pub include_images: Option<bool>,
    /// Emit Markdown link syntax.
    pub include_links: Option<bool>,
}

/// Result of a conversion operation.
#[napi(object)]
pub struct JsConversionResult {
    pub markdown: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

/// Convert an HTML string to Markdown.
///
/// @param html - The HTML to convert.
/// @param options - Optional conversion options.
#[napi]
pub fn convert(html: String, options: Option<JsConversionOptions>) -> JsConversionResult {
    let opts = options.map(|o| ConversionOptions {
        extract_main_content: o.extract_main_content.unwrap_or(true),
        include_images: o.include_images.unwrap_or(true),
        include_links: o.include_links.unwrap_or(true),
        ..Default::default()
    }).unwrap_or_default();

    let result = core_convert(&html, &opts);
    JsConversionResult {
        markdown: result.markdown,
        title: result.title,
        description: result.description,
    }
}
