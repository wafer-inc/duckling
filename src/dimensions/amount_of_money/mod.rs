pub mod en;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct AmountOfMoneyData {
    pub value: f64,
    pub currency: Currency,
    pub precision: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Dollar,
    Cent,
    Euro,
    Pound,
    Yen,
    Won,
    INR,
    AUD,
    CAD,
    HKD,
    Unknown,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::Dollar => "USD",
            Currency::Cent => "cent",
            Currency::Euro => "EUR",
            Currency::Pound => "GBP",
            Currency::Yen => "JPY",
            Currency::Won => "KRW",
            Currency::INR => "INR",
            Currency::AUD => "AUD",
            Currency::CAD => "CAD",
            Currency::HKD => "HKD",
            Currency::Unknown => "unknown",
        }
    }
}

impl AmountOfMoneyData {
    pub fn new(value: f64, currency: Currency) -> Self {
        AmountOfMoneyData {
            value,
            currency,
            precision: None,
        }
    }

    pub fn with_precision(mut self, precision: &str) -> Self {
        self.precision = Some(precision.to_string());
        self
    }
}

pub fn resolve(data: &AmountOfMoneyData) -> ResolvedValue {
    let mut json = serde_json::json!({
        "value": data.value,
        "type": "value",
        "unit": data.currency.as_str(),
    });
    if let Some(ref precision) = data.precision {
        json.as_object_mut()
            .unwrap()
            .insert("precision".to_string(), serde_json::json!(precision));
    }
    ResolvedValue {
        kind: "value".to_string(),
        value: json,
    }
}
