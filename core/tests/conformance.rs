/// Rust integration conformance tests.
///
/// Runs every HTML file in tests/fixtures/inputs/ through the converter
/// (no content extraction) and compares to tests/fixtures/expected/*.md.
///
/// These are the canonical correctness tests. All other language bindings
/// must produce identical output for the same fixtures.
use fast_agent_ingest_core::{convert, ConversionOptions};
use std::fs;
use std::path::Path;

const FIXTURES: &str = "../tests/fixtures";

fn run_fixture(name: &str) {
    let html_path = format!("{}/inputs/{}.html", FIXTURES, name);
    let md_path   = format!("{}/expected/{}.md",  FIXTURES, name);

    let html     = fs::read_to_string(&html_path)
        .unwrap_or_else(|_| panic!("Cannot read {}", html_path));
    let expected = fs::read_to_string(&md_path)
        .unwrap_or_else(|_| panic!("Cannot read {}", md_path));

    let opts = ConversionOptions { extract_main_content: false, ..Default::default() };
    let result = convert(&html, &opts);

    assert_eq!(
        result.markdown.trim(),
        expected.trim(),
        "Markdown mismatch for fixture '{}'", name
    );
}

#[test]
fn fixture_simple_article()  { run_fixture("simple-article"); }

#[test]
fn fixture_code_blocks()     { run_fixture("code-blocks"); }

#[test]
fn fixture_table_heavy()     { run_fixture("table-heavy"); }

#[test]
fn fixture_noisy_page()      { run_fixture("noisy-page"); }

/// Discover and run any fixture not explicitly listed above.
/// New fixtures added to tests/fixtures/ are automatically tested.
#[test]
fn all_fixtures() {
    let inputs = Path::new(FIXTURES).join("inputs");
    if !inputs.exists() { return; }

    for entry in fs::read_dir(&inputs).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map(|e| e == "html").unwrap_or(false) {
            let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
            run_fixture(&name);
        }
    }
}
