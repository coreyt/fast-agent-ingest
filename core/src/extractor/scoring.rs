/// Content score assigned to each block-level element during extraction.
///
/// Implements a simplified version of the Readability/Arc90 algorithm:
///   1. Initialise candidate scores based on tag semantics.
///   2. Add to the score for each content token in the element's text.
///   3. Propagate a fraction of each element's score to its parent.
///   4. Select the highest-scoring candidate as the main content node.
///
/// This module only holds the scoring data structures and helpers.
/// The tree walk that populates them lives in `extractor::mod`.

/// Semantic weight assigned to a tag before text analysis.
pub fn initial_content_score(tag: &str) -> f32 {
    match tag {
        // Strong content containers
        "article" | "main"    => 25.0,
        "section"              => 10.0,
        "div"                  => 5.0,
        // Inline containers — moderate positive signal
        "p" | "td" | "pre"    => 3.0,
        "blockquote"           => 3.0,
        // Typically navigational — penalise
        "address"              => -3.0,
        "ol" | "ul" | "dl"    => -3.0,
        "li"                   => -3.0,
        "form"                 => -50.0,
        "table"                => -1.0,
        "h1" | "h2" | "h3"
        | "h4" | "h5" | "h6"  => 0.0,
        _                      => 0.0,
    }
}

/// Adjust a raw score for link density in a candidate element.
///
/// High link density (anchor text / total text ratio) is a strong signal
/// that an element is navigational or promotional rather than article content.
pub fn apply_link_density_penalty(score: f32, link_density: f32) -> f32 {
    if link_density > 0.5 {
        score * 0.2
    } else if link_density > 0.25 {
        score * 0.7
    } else {
        score
    }
}
