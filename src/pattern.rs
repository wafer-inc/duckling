use regex::Regex;
use crate::types::{DimensionKind, PatternItem, TokenData};

/// Create a regex pattern item. The pattern is matched case-insensitively
/// against the lowercased document text.
pub fn regex(pattern: &str) -> PatternItem {
    let full = format!("(?i){}", pattern);
    PatternItem::Regex(Regex::new(&full).unwrap_or_else(|e| {
        panic!("Invalid regex pattern '{}': {}", pattern, e)
    }))
}

/// Create a regex pattern item with an exact regex (no auto case-insensitive flag).
pub fn regex_exact(pattern: &str) -> PatternItem {
    PatternItem::Regex(Regex::new(pattern).unwrap_or_else(|e| {
        panic!("Invalid regex pattern '{}': {}", pattern, e)
    }))
}

/// Create a dimension pattern item that matches any token of the given dimension.
pub fn dim(kind: DimensionKind) -> PatternItem {
    PatternItem::Dimension(kind)
}

/// Create a predicate pattern item with a custom matching function.
pub fn predicate<F>(f: F) -> PatternItem
where
    F: Fn(&TokenData) -> bool + Send + Sync + 'static,
{
    PatternItem::Predicate(Box::new(f))
}
