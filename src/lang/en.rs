use crate::dimensions;
use crate::locale::Region;
use crate::types::{DimensionKind, Rule};

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

/// EN default rules when no locale is specified.
pub(crate) fn default_rules(needed: &[DimensionKind]) -> Vec<Rule> {
    // Haskell EN default rules include standard EN language rules.
    lang_rules(needed)
}

/// EN language rules.
pub(crate) fn lang_rules(needed: &[DimensionKind]) -> Vec<Rule> {
    let mut rules = Vec::new();
    for dim in needed {
        match dim {
            DimensionKind::Numeral => rules.extend(dimensions::numeral::en::lang_rules()),
            DimensionKind::Ordinal => rules.extend(dimensions::ordinal::en::rules()),
            DimensionKind::Temperature => rules.extend(dimensions::temperature::en::rules()),
            DimensionKind::Distance => rules.extend(dimensions::distance::en::lang_rules()),
            DimensionKind::Volume => rules.extend(dimensions::volume::en::rules()),
            DimensionKind::Quantity => rules.extend(dimensions::quantity::en::rules()),
            DimensionKind::AmountOfMoney => {
                rules.extend(dimensions::amount_of_money::en::lang_rules())
            }
            DimensionKind::Email => {}
            DimensionKind::PhoneNumber => {}
            DimensionKind::Url => {}
            DimensionKind::CreditCardNumber => {}
            DimensionKind::TimeGrain => rules.extend(dimensions::time_grain::en::rules()),
            DimensionKind::Duration => rules.extend(dimensions::duration::en::lang_rules()),
            DimensionKind::Time => rules.extend(dimensions::time::en::rules()),
        }
    }

    rules
}

/// Region-specific rule overlays for English locales.
/// No region overlays are implemented yet, so this currently returns empty.
pub(crate) fn locale_rules(_region: Option<Region>, _needed: &[DimensionKind]) -> Vec<Rule> {
    Vec::new()
}
