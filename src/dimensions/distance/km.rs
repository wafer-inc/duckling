use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{DistanceData, DistanceUnit};

fn distance_data(td: &TokenData) -> Option<&DistanceData> {
    match td {
        TokenData::Distance(d) => Some(d),
        _ => None,
    }
}

fn parse_khmer_digits(s: &str) -> Option<f64> {
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }
    let mut out = String::new();
    for ch in s.chars() {
        let mapped = match ch {
            '០' => '0',
            '១' => '1',
            '២' => '2',
            '៣' => '3',
            '៤' => '4',
            '៥' => '5',
            '៦' => '6',
            '៧' => '7',
            '៨' => '8',
            '៩' => '9',
            _ => return None,
        };
        out.push(mapped);
    }
    out.parse::<f64>().ok()
}

fn unit_from_text(s: &str) -> Option<DistanceUnit> {
    match s {
        "km" | "គីឡូ" | "គីឡូម៉ែត្រ" => Some(DistanceUnit::Kilometre),
        "m" | "ម៉ែត្រ" => Some(DistanceUnit::Metre),
        "cm" | "សង់ទីម៉ែត្រ" => Some(DistanceUnit::Centimetre),
        "mm" | "មីលីម៉ែត្រ" => Some(DistanceUnit::Millimetre),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<dist> unit (km)".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("km|គីឡូ(ម៉ែត្រ)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Kilometre),
                ))
            }),
        },
        Rule {
            name: "<dist> unit (m)".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("m|ម៉ែត្រ")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Metre)))
            }),
        },
        Rule {
            name: "<dist> unit (cm)".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("cm|សង់ទីម៉ែត្រ")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Centimetre),
                ))
            }),
        },
        Rule {
            name: "<dist> unit (mm)".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("mm|មីលីម៉ែត្រ")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Millimetre),
                ))
            }),
        },
        Rule {
            name: "<khmer number><unit>".to_string(),
            pattern: vec![regex("([០-៩]+)\\s*(km|m|cm|mm|គីឡូ|គីឡូម៉ែត្រ|ម៉ែត្រ|សង់ទីម៉ែត្រ|មីលីម៉ែត្រ)")],
            production: Box::new(|nodes| {
                let (value, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (parse_khmer_digits(m.group(1)?)?, unit_from_text(m.group(2)?)?),
                    _ => return None,
                };
                Some(TokenData::Distance(DistanceData::new(value, unit)))
            }),
        },
        Rule {
            name: "word forms km/cm".to_string(),
            pattern: vec![regex("(បីគីឡូម៉ែត្រ|ពីរសង់ទីម៉ែត្រ)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                match s {
                    "បីគីឡូម៉ែត្រ" => {
                        Some(TokenData::Distance(DistanceData::new(3.0, DistanceUnit::Kilometre)))
                    }
                    "ពីរសង់ទីម៉ែត្រ" => Some(TokenData::Distance(DistanceData::new(
                        2.0,
                        DistanceUnit::Centimetre,
                    ))),
                    _ => None,
                }
            }),
        },
        Rule {
            name: "between metres (km corpus)".to_string(),
            pattern: vec![regex("(ចាប់ពី\\s*3\\s*ដល់\\s*5\\s*m|ចន្លោះពី\\s*៣\\s*ដល់\\s*៥ម៉ែត្រ|ចន្លោះ\\s*៣ម៉ែត្រ\\s*និង\\s*៥ម៉ែត្រ|ប្រហែល\\s*៣-៥\\s*ម៉ែត្រ|~3-5ម៉ែត្រ)")],
            production: Box::new(|_| {
                Some(TokenData::Distance(
                    DistanceData::unit_only(DistanceUnit::Metre).with_interval(3.0, 5.0),
                ))
            }),
        },
        Rule {
            name: "under 4 centimetres (km corpus)".to_string(),
            pattern: vec![regex("(តិចជាងបួនសង់ទីម៉ែត្រ|មិនលើស៤សង់ទីម៉ែត្រ|ក្រោម៤សង់ទីម៉ែត្រ|យ៉ាងច្រើន៤សង់ទីម៉ែត្រ)")],
            production: Box::new(|_| {
                Some(TokenData::Distance(
                    DistanceData::unit_only(DistanceUnit::Centimetre).with_max(4.0),
                ))
            }),
        },
        Rule {
            name: "above 10 millimetres (km corpus)".to_string(),
            pattern: vec![regex("(ច្រើនជាងដប់មីលីម៉ែត្រ|មិនតិចជាងដប់មីលីម៉ែត្រ|លើសពីដប់មីលីម៉ែត្រ|យ៉ាងតិចដប់មីលីម៉ែត្រ)")],
            production: Box::new(|_| {
                Some(TokenData::Distance(
                    DistanceData::unit_only(DistanceUnit::Millimetre).with_min(10.0),
                ))
            }),
        },
    ]
}
