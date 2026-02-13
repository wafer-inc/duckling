pub mod af;
pub mod ar;
pub mod bn;
pub mod cs;
pub mod en;
pub mod es;
pub mod bg;
pub mod ca;
pub mod da;
pub mod de;
pub mod et;
pub mod el;
pub mod fa;
pub mod fi;
pub mod ga;
pub mod fr;
pub mod he;
pub mod hi;
pub mod hr;
pub mod hu;
pub mod id;
pub mod is;
pub mod it;
pub mod ja;
pub mod ka;
pub mod kn;
pub mod ko;
pub mod km;
pub mod lo;
pub mod ml;
pub mod mn;
pub mod my;
pub mod nb;
pub mod ne;
pub mod nl;
pub mod pt;
pub mod pl;
pub mod ro;
pub mod ru;
pub mod sk;
pub mod sv;
pub mod sw;
pub mod ta;
pub mod th;
pub mod te;
pub mod tr;
pub mod uk;
pub mod vi;
pub mod zh;
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
