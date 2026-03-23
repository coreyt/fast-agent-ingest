use html5ever::{parse_document, ParseOpts};
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;

/// Parse an HTML string into an RcDom tree using a WHATWG-spec-compliant
/// html5ever parser. Handles malformed real-world HTML gracefully.
pub fn parse(html: &str) -> RcDom {
    parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap()
}
