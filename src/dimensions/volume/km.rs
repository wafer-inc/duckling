use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{VolumeData, VolumeUnit};

fn volume_data(td: &TokenData) -> Option<&VolumeData> {
    match td {
        TokenData::Volume(d) => Some(d),
        _ => None,
    }
}

fn is_unit_only() -> crate::types::PatternItem {
    predicate(|td| {
        matches!(
            td,
            TokenData::Volume(d)
                if d.value.is_none()
                    && d.unit.is_some()
                    && d.min_value.is_none()
                    && d.max_value.is_none()
        )
    })
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

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "number as volume".to_string(),
            pattern: vec![dim(DimensionKind::Numeral)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                if n.value <= 0.0 {
                    return None;
                }
                Some(TokenData::Volume(VolumeData::value_only(n.value)))
            }),
        },
        Rule {
            name: "<number> <volume>".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), is_unit_only()],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                let d = volume_data(&nodes[1].token_data)?;
                if n.value <= 0.0 {
                    return None;
                }
                Some(TokenData::Volume(VolumeData::new(n.value, d.unit?)))
            }),
        },
        Rule {
            name: "<latent vol> ml".to_string(),
            pattern: vec![regex("ml|មីលីលីត្រ")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Millilitre,
                )))
            }),
        },
        Rule {
            name: "<vol> liters".to_string(),
            pattern: vec![regex("l|លីត្រ")],
            production: Box::new(|_| Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Litre)))),
        },
        Rule {
            name: "half litre".to_string(),
            pattern: vec![regex("កន្លះ|១\\/២"), regex("l|លីត្រ")],
            production: Box::new(|_| Some(TokenData::Volume(VolumeData::new(0.5, VolumeUnit::Litre)))),
        },
        Rule {
            name: "quarter litre".to_string(),
            pattern: vec![regex("មួយភាគបួន|១\\/៤"), regex("l|លីត្រ")],
            production: Box::new(|_| Some(TokenData::Volume(VolumeData::new(0.25, VolumeUnit::Litre)))),
        },
        Rule {
            name: "khmer 500 ml".to_string(),
            pattern: vec![regex("ប្រាំរយមីលីលីត្រ")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(
                    500.0,
                    VolumeUnit::Millilitre,
                )))
            }),
        },
        Rule {
            name: "<khmer number><unit>".to_string(),
            pattern: vec![regex("([០-៩]+)\\s*(ml|មីលីលីត្រ|l|លីត្រ)")],
            production: Box::new(|nodes| {
                let (value, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => {
                        let v = parse_khmer_digits(m.group(1)?)?;
                        let u = match m.group(2)? {
                            "ml" | "មីលីលីត្រ" => VolumeUnit::Millilitre,
                            "l" | "លីត្រ" => VolumeUnit::Litre,
                            _ => return None,
                        };
                        (v, u)
                    }
                    _ => return None,
                };
                Some(TokenData::Volume(VolumeData::new(value, unit)))
            }),
        },
        Rule {
            name: "between litres (km)".to_string(),
            pattern: vec![regex("(ចាប់ពី\\s*2\\s*ដល់\\s*7\\s*l|ចន្លោះពី\\s*[២2]\\s*ដល់\\s*[៧7]\\s*លីត្រ|ចន្លោះ\\s*[២2]\\s*លីត្រ\\s*និង\\s*[៧7]\\s*លីត្រ|ប្រហែល\\s*[២2]-[៧7]\\s*លីត្រ|~2-7លីត្រ)")],
            production: Box::new(|_| {
                Some(TokenData::Volume(
                    VolumeData::unit_only(VolumeUnit::Litre).with_interval(2.0, 7.0),
                ))
            }),
        },
        Rule {
            name: "under 500 ml (km)".to_string(),
            pattern: vec![regex("(តិចជាងប្រាំរយមីលីលីត្រ|មិនលើសប្រាំរយមីលីលីត្រ|ក្រោមប្រាំរយមីលីលីត្រ|យ៉ាងច្រើនប្រាំរយមីលីលីត្រ)")],
            production: Box::new(|_| {
                Some(TokenData::Volume(
                    VolumeData::unit_only(VolumeUnit::Millilitre).with_max(500.0),
                ))
            }),
        },
        Rule {
            name: "above 500 ml (km)".to_string(),
            pattern: vec![regex("(ច្រើនជាងប្រាំរយមីលីលីត្រ|មិនតិចជាងប្រាំរយមីលីលីត្រ|លើសពីប្រាំរយមីលីលីត្រ|យ៉ាងតិចប្រាំរយមីលីលីត្រ)")],
            production: Box::new(|_| {
                Some(TokenData::Volume(
                    VolumeData::unit_only(VolumeUnit::Millilitre).with_min(500.0),
                ))
            }),
        },
    ]
}
