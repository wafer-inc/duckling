pub mod amount_of_money;
pub mod credit_card_number;
pub mod distance;
pub mod duration;
pub mod email;
pub mod numeral;
pub mod ordinal;
pub mod phone_number;
pub mod quantity;
pub mod temperature;
pub mod time;
pub mod time_grain;
pub mod url;
pub mod volume;

use crate::types::DimensionKind;

/// Returns the dimensions that a given dimension depends on.
pub fn dimension_dependencies(dim: DimensionKind) -> Vec<DimensionKind> {
    match dim {
        DimensionKind::Temperature => vec![DimensionKind::Numeral],
        DimensionKind::Distance => vec![DimensionKind::Numeral],
        DimensionKind::Volume => vec![DimensionKind::Numeral],
        DimensionKind::Quantity => vec![DimensionKind::Numeral],
        DimensionKind::AmountOfMoney => vec![DimensionKind::Numeral],
        DimensionKind::Duration => vec![DimensionKind::Numeral, DimensionKind::TimeGrain],
        DimensionKind::Time => vec![
            DimensionKind::Numeral,
            DimensionKind::Ordinal,
            DimensionKind::Duration,
            DimensionKind::TimeGrain,
        ],
        _ => vec![],
    }
}
