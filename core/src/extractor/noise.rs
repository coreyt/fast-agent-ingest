/// Tags whose content should always be removed wholesale — they carry no
/// text content useful to an LLM.
pub const NOISE_TAGS: &[&str] = &[
    "script", "style", "noscript", "iframe", "object", "embed",
    "svg",    "canvas", "video",   "audio",  "form",   "button",
    "input",  "select", "textarea","footer", "nav",    "aside",
    "header",
];

/// Heuristic class/id substrings that strongly indicate advertising or
/// navigational chrome rather than article content.
pub const NOISE_PATTERNS: &[&str] = &[
    "nav",       "navbar",    "navigation",
    "sidebar",   "side-bar",  "widget",
    "ad",        "ads",       "advert",    "advertisement", "sponsored",
    "banner",    "promo",     "popup",     "modal",
    "cookie",    "gdpr",
    "share",     "social",
    "comment",   "comments",  "disqus",
    "related",   "recommend", "suggested",
    "footer",    "footnote",
    "breadcrumb","pagination","pager",
    "menu",      "toc",       "table-of-contents",
];

/// Returns `true` if the element with the given tag name should be dropped
/// entirely, regardless of its attributes.
pub fn is_noise_tag(tag: &str) -> bool {
    NOISE_TAGS.contains(&tag)
}

/// Returns `true` if `value` (a class or id string) contains a noise pattern.
pub fn is_noise_attr(value: &str) -> bool {
    let lower = value.to_lowercase();
    NOISE_PATTERNS.iter().any(|p| lower.contains(p))
}
