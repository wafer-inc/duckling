pub mod en;
pub mod helpers;

use crate::types::DimensionValue;

#[derive(Debug, Clone)]
pub struct NumeralData {
    pub value: f64,
    pub grain: Option<u8>,
    pub multipliable: bool,
    /// True for quantifier words like "single", "couple", "few", "dozen"
    /// that should not be interpreted as clock hours.
    pub quantifier: bool,
}

impl NumeralData {
    pub fn new(value: f64) -> Self {
        NumeralData {
            value,
            grain: None,
            multipliable: false,
            quantifier: false,
        }
    }

    pub fn with_grain(mut self, grain: u8) -> Self {
        self.grain = Some(grain);
        self
    }

    pub fn with_multipliable(mut self, multipliable: bool) -> Self {
        self.multipliable = multipliable;
        self
    }

    pub fn with_quantifier(mut self) -> Self {
        self.quantifier = true;
        self
    }
}

pub fn resolve(data: &NumeralData) -> DimensionValue {
    DimensionValue::Numeral(data.value)
}
