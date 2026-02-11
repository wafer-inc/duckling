pub mod en;

use crate::types::ResolvedValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    // Ambiguous
    Cent,
    Dinar,
    Dirham,
    Dollar,
    Pound,
    Rial,
    Riyal,
    Unnamed,
    // Specific
    AED,
    AUD,
    BGN,
    BRL,
    BYN,
    CAD,
    CHF,
    CNY,
    CZK,
    DKK,
    EGP,
    EUR,
    GBP,
    GEL,
    HKD,
    HRK,
    IDR,
    ILS,
    INR,
    IQD,
    JMD,
    JOD,
    JPY,
    KRW,
    KWD,
    LBP,
    MAD,
    MNT,
    MYR,
    NOK,
    NZD,
    PKR,
    PLN,
    PTS,
    QAR,
    RON,
    RUB,
    SAR,
    SEK,
    SGD,
    THB,
    TTD,
    UAH,
    USD,
    VND,
    ZAR,
    TRY,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::Cent => "cent",
            Currency::Dinar => "dinar",
            Currency::Dirham => "dirham",
            Currency::Dollar => "USD",
            Currency::Pound => "GBP",
            Currency::Rial => "rial",
            Currency::Riyal => "riyal",
            Currency::Unnamed => "USD",
            Currency::AED => "AED",
            Currency::AUD => "AUD",
            Currency::BGN => "BGN",
            Currency::BRL => "BRL",
            Currency::BYN => "BYN",
            Currency::CAD => "CAD",
            Currency::CHF => "CHF",
            Currency::CNY => "CNY",
            Currency::CZK => "CZK",
            Currency::DKK => "DKK",
            Currency::EGP => "EGP",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::GEL => "GEL",
            Currency::HKD => "HKD",
            Currency::HRK => "HRK",
            Currency::IDR => "IDR",
            Currency::ILS => "ILS",
            Currency::INR => "INR",
            Currency::IQD => "IQD",
            Currency::JMD => "JMD",
            Currency::JOD => "JOD",
            Currency::JPY => "JPY",
            Currency::KRW => "KRW",
            Currency::KWD => "KWD",
            Currency::LBP => "LBP",
            Currency::MAD => "MAD",
            Currency::MNT => "MNT",
            Currency::MYR => "MYR",
            Currency::NOK => "NOK",
            Currency::NZD => "NZD",
            Currency::PKR => "PKR",
            Currency::PLN => "PLN",
            Currency::PTS => "PTS",
            Currency::QAR => "QAR",
            Currency::RON => "RON",
            Currency::RUB => "RUB",
            Currency::SAR => "SAR",
            Currency::SEK => "SEK",
            Currency::SGD => "SGD",
            Currency::THB => "THB",
            Currency::TTD => "TTD",
            Currency::UAH => "UAH",
            Currency::USD => "USD",
            Currency::VND => "VND",
            Currency::ZAR => "ZAR",
            Currency::TRY => "TRY",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AmountOfMoneyData {
    pub value: Option<f64>,
    pub currency: Currency,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub latent: bool,
}

impl AmountOfMoneyData {
    pub fn currency_only(c: Currency) -> Self {
        AmountOfMoneyData {
            value: None,
            currency: c,
            min_value: None,
            max_value: None,
            latent: false,
        }
    }

    pub fn with_value(mut self, v: f64) -> Self {
        self.value = Some(v);
        self
    }

    pub fn with_cents(self, c: f64) -> Self {
        match self.value {
            Some(v) => AmountOfMoneyData {
                value: Some(v + c / 100.0),
                ..self
            },
            None => AmountOfMoneyData {
                value: Some(c),
                currency: Currency::Cent,
                min_value: None,
                max_value: None,
                latent: false,
            },
        }
    }

    pub fn with_interval(mut self, from: f64, to: f64) -> Self {
        self.min_value = Some(from);
        self.max_value = Some(to);
        self.value = None;
        self
    }

    pub fn with_min(mut self, v: f64) -> Self {
        self.min_value = Some(v);
        self
    }

    pub fn with_max(mut self, v: f64) -> Self {
        self.max_value = Some(v);
        self
    }

    pub fn mk_latent(mut self) -> Self {
        self.latent = true;
        self
    }
}

pub fn resolve(data: &AmountOfMoneyData, with_latent: bool) -> Option<ResolvedValue> {
    // Latent tokens filtered when with_latent=false
    if data.latent && !with_latent {
        return None;
    }
    // Currency-only tokens don't resolve
    if data.value.is_none() && data.min_value.is_none() && data.max_value.is_none() {
        return None;
    }

    let unit = data.currency.as_str();

    if let Some(value) = data.value {
        // Simple value
        Some(ResolvedValue {
            kind: "value".to_string(),
            value: serde_json::json!({
                "value": value,
                "type": "value",
                "unit": unit,
            }),
        })
    } else if let (Some(from), Some(to)) = (data.min_value, data.max_value) {
        // Between interval
        Some(ResolvedValue {
            kind: "value".to_string(),
            value: serde_json::json!({
                "type": "interval",
                "value": from,
                "unit": unit,
                "from": {"value": from, "unit": unit},
                "to": {"value": to, "unit": unit},
            }),
        })
    } else if let Some(from) = data.min_value {
        // Above / at least
        Some(ResolvedValue {
            kind: "value".to_string(),
            value: serde_json::json!({
                "type": "interval",
                "value": from,
                "unit": unit,
                "from": {"value": from, "unit": unit},
            }),
        })
    } else if let Some(to) = data.max_value {
        // Under / at most
        Some(ResolvedValue {
            kind: "value".to_string(),
            value: serde_json::json!({
                "type": "interval",
                "value": to,
                "unit": unit,
                "to": {"value": to, "unit": unit},
            }),
        })
    } else {
        None
    }
}
