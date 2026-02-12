use std::sync::OnceLock;

use crate::dimensions;
use crate::types::{DimensionKind, Rule};

/// Cached full rule set (all dimensions) for English.
static ALL_RULES: OnceLock<Vec<Rule>> = OnceLock::new();

/// Get all compiled rules for English, cached after first compilation.
pub fn all_rules() -> &'static [Rule] {
    ALL_RULES.get_or_init(|| rules_uncached(&[]))
}

pub fn supported_dimensions() -> Vec<DimensionKind> {
    vec![
        DimensionKind::Numeral,
        DimensionKind::Ordinal,
        DimensionKind::Temperature,
        DimensionKind::Distance,
        DimensionKind::Volume,
        DimensionKind::Quantity,
        DimensionKind::AmountOfMoney,
        DimensionKind::Email,
        DimensionKind::PhoneNumber,
        DimensionKind::Url,
        DimensionKind::CreditCardNumber,
        DimensionKind::TimeGrain,
        DimensionKind::Duration,
        DimensionKind::Time,
    ]
}

/// Collect all rules needed for the given dimensions in English.
/// Automatically includes dependency dimensions.
fn rules_uncached(dims: &[DimensionKind]) -> Vec<Rule> {
    let mut needed: Vec<DimensionKind> = Vec::new();

    // Add requested dims and their dependencies
    for dim in dims {
        add_with_deps(*dim, &mut needed);
    }

    // If no specific dims requested, include all
    if needed.is_empty() {
        needed = supported_dimensions();
    }

    let mut rules = Vec::new();

    for dim in &needed {
        match dim {
            DimensionKind::Numeral => rules.extend(dimensions::numeral::en::rules()),
            DimensionKind::Ordinal => rules.extend(dimensions::ordinal::en::rules()),
            DimensionKind::Temperature => rules.extend(dimensions::temperature::en::rules()),
            DimensionKind::Distance => rules.extend(dimensions::distance::en::rules()),
            DimensionKind::Volume => rules.extend(dimensions::volume::en::rules()),
            DimensionKind::Quantity => rules.extend(dimensions::quantity::en::rules()),
            DimensionKind::AmountOfMoney => rules.extend(dimensions::amount_of_money::en::rules()),
            DimensionKind::Email => rules.extend(dimensions::email::rules::rules()),
            DimensionKind::PhoneNumber => rules.extend(dimensions::phone_number::rules::rules()),
            DimensionKind::Url => rules.extend(dimensions::url::rules::rules()),
            DimensionKind::CreditCardNumber => {
                rules.extend(dimensions::credit_card_number::rules::rules())
            }
            DimensionKind::TimeGrain => rules.extend(dimensions::time_grain::en::rules()),
            DimensionKind::Duration => rules.extend(dimensions::duration::en::rules()),
            DimensionKind::Time => rules.extend(dimensions::time::en::rules()),
        }
    }

    rules
}

fn add_with_deps(dim: DimensionKind, needed: &mut Vec<DimensionKind>) {
    if needed.contains(&dim) {
        return;
    }
    // Add dependencies first
    for dep in dimensions::dimension_dependencies(dim) {
        add_with_deps(dep, needed);
    }
    needed.push(dim);
}
