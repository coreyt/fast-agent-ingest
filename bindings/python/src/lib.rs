use pyo3::prelude::*;
use fast_agent_ingest_core::{convert as core_convert, ConversionOptions};

/// Options controlling HTML → Markdown conversion.
#[pyclass(name = "ConversionOptions")]
#[derive(Clone)]
pub struct PyConversionOptions {
    #[pyo3(get, set)]
    pub extract_main_content: bool,
    #[pyo3(get, set)]
    pub include_images: bool,
    #[pyo3(get, set)]
    pub include_links: bool,
}

#[pymethods]
impl PyConversionOptions {
    #[new]
    #[pyo3(signature = (extract_main_content=true, include_images=true, include_links=true))]
    fn new(extract_main_content: bool, include_images: bool, include_links: bool) -> Self {
        Self { extract_main_content, include_images, include_links }
    }
}

impl From<&PyConversionOptions> for ConversionOptions {
    fn from(py: &PyConversionOptions) -> Self {
        ConversionOptions {
            extract_main_content: py.extract_main_content,
            include_images: py.include_images,
            include_links: py.include_links,
            ..Default::default()
        }
    }
}

/// Result of a conversion operation.
#[pyclass(name = "ConversionResult")]
pub struct PyConversionResult {
    #[pyo3(get)]
    pub markdown: String,
    #[pyo3(get)]
    pub title: Option<String>,
    #[pyo3(get)]
    pub description: Option<String>,
}

/// Convert an HTML string to Markdown.
///
/// Args:
///     html: The HTML string to convert.
///     options: Optional ConversionOptions instance. Defaults to sensible values.
///
/// Returns:
///     ConversionResult with `.markdown`, `.title`, and `.description`.
#[pyfunction]
#[pyo3(signature = (html, options=None))]
fn convert(html: &str, options: Option<&PyConversionOptions>) -> PyResult<PyConversionResult> {
    let opts = options
        .map(ConversionOptions::from)
        .unwrap_or_default();

    let result = core_convert(html, &opts);
    Ok(PyConversionResult {
        markdown: result.markdown,
        title: result.title,
        description: result.description,
    })
}

#[pymodule]
fn fast_agent_ingest(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyConversionOptions>()?;
    m.add_class::<PyConversionResult>()?;
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    Ok(())
}
