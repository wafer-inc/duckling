pub mod en;

use crate::locale::Lang;
use crate::types::{DimensionKind, Rule};

/// Get rules for a given language and set of dimensions.
/// Rules are cached after first compilation to avoid repeated regex compilation.
pub fn rules_for(lang: Lang, _dims: &[DimensionKind]) -> &'static [Rule] {
    match lang {
        Lang::EN => en::all_rules(),
    }
}

/// Get supported dimensions for a language.
pub fn supported_dimensions(lang: Lang) -> Vec<DimensionKind> {
    match lang {
        Lang::EN => en::supported_dimensions(),
    }
}
