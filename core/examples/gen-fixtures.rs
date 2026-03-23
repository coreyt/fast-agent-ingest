use fast_agent_ingest_core::{convert, ConversionOptions};
use std::fs;

fn main() {
    let fixtures = ["simple-article", "code-blocks", "table-heavy", "noisy-page"];
    let opts = ConversionOptions { extract_main_content: false, ..Default::default() };

    for name in fixtures {
        let html = fs::read_to_string(format!("tests/fixtures/inputs/{}.html", name)).unwrap();
        let result = convert(&html, &opts);
        let path = format!("tests/fixtures/expected/{}.md", name);
        fs::create_dir_all("tests/fixtures/expected").unwrap();
        fs::write(&path, &result.markdown).unwrap();
        println!("Generated: {}", path);
    }
}
