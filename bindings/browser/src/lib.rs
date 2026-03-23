use wasm_bindgen::prelude::*;
use fast_agent_ingest_core::{convert as core_convert, ConversionOptions};

/// Options controlling HTML → Markdown conversion.
#[wasm_bindgen]
pub struct WasmConversionOptions {
    extract_main_content: bool,
    include_images: bool,
    include_links: bool,
}

#[wasm_bindgen]
impl WasmConversionOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            extract_main_content: true,
            include_images: true,
            include_links: true,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn extract_main_content(&self) -> bool { self.extract_main_content }
    #[wasm_bindgen(setter)]
    pub fn set_extract_main_content(&mut self, v: bool) { self.extract_main_content = v; }

    #[wasm_bindgen(getter)]
    pub fn include_images(&self) -> bool { self.include_images }
    #[wasm_bindgen(setter)]
    pub fn set_include_images(&mut self, v: bool) { self.include_images = v; }

    #[wasm_bindgen(getter)]
    pub fn include_links(&self) -> bool { self.include_links }
    #[wasm_bindgen(setter)]
    pub fn set_include_links(&mut self, v: bool) { self.include_links = v; }
}

/// Result of a conversion operation.
#[wasm_bindgen]
pub struct WasmConversionResult {
    markdown: String,
    title: Option<String>,
    description: Option<String>,
}

#[wasm_bindgen]
impl WasmConversionResult {
    #[wasm_bindgen(getter)]
    pub fn markdown(&self) -> String { self.markdown.clone() }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> Option<String> { self.title.clone() }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> Option<String> { self.description.clone() }
}

/// Convert an HTML string to Markdown.
///
/// @param html - The HTML to convert.
/// @param options - Optional conversion options.
#[wasm_bindgen]
pub fn convert(html: &str, options: Option<WasmConversionOptions>) -> WasmConversionResult {
    let opts = options.map(|o| ConversionOptions {
        extract_main_content: o.extract_main_content,
        include_images: o.include_images,
        include_links: o.include_links,
        ..Default::default()
    }).unwrap_or_default();

    let result = core_convert(html, &opts);
    WasmConversionResult {
        markdown: result.markdown,
        title: result.title,
        description: result.description,
    }
}
