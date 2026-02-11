pub mod en;

use crate::locale::Lang;
use crate::types::{DimensionKind, Rule};

/// Get rules for a given language and set of dimensions.
pub fn rules_for(lang: Lang, dims: &[DimensionKind]) -> Vec<Rule> {
    match lang {
        Lang::EN => en::rules(dims),
    }
}

/// Get supported dimensions for a language.
pub fn supported_dimensions(lang: Lang) -> Vec<DimensionKind> {
    match lang {
        Lang::EN => en::supported_dimensions(),
    }
}
