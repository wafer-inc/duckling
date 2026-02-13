use crate::dimensions::numeral::helpers::{decimals_to_double, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::NumeralData;

fn arabic_digit_to_ascii(c: char) -> char {
    match c {
        '٠' => '0',
        '١' => '1',
        '٢' => '2',
        '٣' => '3',
        '٤' => '4',
        '٥' => '5',
        '٦' => '6',
        '٧' => '7',
        '٨' => '8',
        '٩' => '9',
        _ => c,
    }
}

fn parse_arabic_number(raw: &str) -> Option<f64> {
    let normalized: String = raw
        .chars()
        .map(arabic_digit_to_ascii)
        .filter(|c| *c != '٬' && *c != ',')
        .map(|c| if c == '٫' { '.' } else { c })
        .collect();
    normalized.parse::<f64>().ok()
}

fn lex_unit(s: &str) -> Option<f64> {
    match s {
        "صفر" => Some(0.0),
        "واحد" | "واحدة" => Some(1.0),
        "إثنين" | "اثنين" | "إثنتين" | "اثنتين" | "اتنين" => Some(2.0),
        "ثلاثة" | "ثلاث" | "تلاتة" => Some(3.0),
        "أربعة" | "اربعة" | "أربع" | "اربع" | "اربعه" => Some(4.0),
        "خمسة" | "خمس" => Some(5.0),
        "ستة" | "ست" | "سته" => Some(6.0),
        "سبعة" | "سبع" => Some(7.0),
        "ثمانية" | "ثمان" | "تمنية" | "تمنة" => Some(8.0),
        "تسعة" | "تسع" => Some(9.0),
        "عشرة" | "عشره" | "عشر" => Some(10.0),
        _ => None,
    }
}

fn tens_word(s: &str) -> Option<f64> {
    match s {
        "عشرون" | "عشرين" => Some(20.0),
        "ثلاثون" | "ثلاثين" | "تلاتين" => Some(30.0),
        "أربعون" | "اربعون" | "أربعين" | "اربعين" => Some(40.0),
        "خمسون" | "خمسين" => Some(50.0),
        "ستون" | "ستين" => Some(60.0),
        "سبعون" | "سبعين" => Some(70.0),
        "ثمانون" | "تمانين" | "ثمانين" => Some(80.0),
        "تسعون" | "تسعين" => Some(90.0),
        _ => None,
    }
}

fn hundred_word(s: &str) -> Option<f64> {
    match s {
        "مائة" | "مائه" | "مئة" | "مئه" => Some(100.0),
        "مائتين" | "مائتان" | "متين" => Some(200.0),
        "ثلاثمائة" | "ثلاثماية" | "تلتمية" => Some(300.0),
        "أربعمائة" | "اربعمائة" | "أربعمية" | "اربعمية" => Some(400.0),
        "خمسمائة" | "خمسمية" => Some(500.0),
        "ستمائة" | "ستمية" => Some(600.0),
        "سبعمائة" | "سبعمية" => Some(700.0),
        "ثمانمائة" | "ثمانمية" => Some(800.0),
        "تسعمائة" | "تسعمية" => Some(900.0),
        _ => None,
    }
}

fn teens_special(s: &str) -> Option<f64> {
    match s {
        "إحدى عشرة" | "إحدى عشر" | "احد عشر" | "حداشر" => Some(11.0),
        "إثنتى عشر" | "إثنى عشر" | "اتناشر" | "إتناشر" => Some(12.0),
        "اربعتاشر" | "أربعتاشر" => Some(14.0),
        "ستاشر" => Some(16.0),
        "سبعتاشر" => Some(17.0),
        "تمنتاشر" => Some(18.0),
        _ => None,
    }
}

fn power_word(s: &str) -> Option<NumeralData> {
    match s {
        "ألف" | "الف" | "آلاف" | "الاف" => {
            Some(NumeralData::new(1e3).with_grain(3).with_multipliable(true))
        }
        "الفين" | "الفان" => Some(NumeralData::new(2e3).with_grain(3)),
        "مليون" | "ملايين" => Some(NumeralData::new(1e6).with_grain(6).with_multipliable(true)),
        "مليونين" | "مليونان" => Some(NumeralData::new(2e6).with_grain(6)),
        _ => None,
    }
}

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "Arabic numeric (with separators)".to_string(),
            pattern: vec![regex("([٠-٩]+(?:[٬,][٠-٩]{3})*(?:[٫.][٠-٩]+)?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(parse_arabic_number(s)?)))
            }),
        },
        Rule {
            name: "Arabic fractional number numeric".to_string(),
            pattern: vec![regex("([٠-٩]+)/([٠-٩]+)")],
            production: Box::new(|nodes| {
                let (n, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let nn = parse_arabic_number(n)?;
                let dd = parse_arabic_number(d)?;
                if dd == 0.0 {
                    None
                } else {
                    Some(TokenData::Numeral(NumeralData::new(nn / dd)))
                }
            }),
        },
        Rule {
            name: "integer 0..10".to_string(),
            pattern: vec![regex("(صفر|واحد[ةه]?|[إا]ثن(ين|تين)|اتنين|ثلاث[ةه]?|[أا]ربع[ةه]?|خمس[ةه]?|ست[ةه]?|سبع[ةه]?|ثما?ني?[ةه]?|عشر[ةه]?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(lex_unit(&s)?)))
            }),
        },
        Rule {
            name: "integer 11..18 special".to_string(),
            pattern: vec![regex("(حداشر|[إاأ]حد[يى]? عشر[ةه]?|إتناشر|اتناشر|[إا]?ت?ث?ن(ت)?[يىا] ?عشر[ةه]?|ا?ربعتاشر|ستاشر|سبعتاشر|تمنتاشر)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(teens_special(&s)?)))
            }),
        },
        Rule {
            name: "integer (13..19)".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if (3.0..10.0).contains(&d.value))),
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value == 10.0)),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v + 10.0)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("((عشر|ثلاث|[أا]ربع|خمس|ست|سبع|ثمان|تسع)(ون|ين)|تلاتين)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens_word(&s)?)))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..10.0).contains(&d.value))),
                regex("و"),
                predicate(|td| matches!(td, TokenData::Numeral(d) if [20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "hundreds lex".to_string(),
            pattern: vec![regex("(مائت(ين|ان)|متين|ثلاثمائة|ثلاثماية|تلتمية|[أا]ربعم(ائة|ية)|خمسم(ائة|ية)|ستم(ائة|ية)|سبعم(ائة|ية)|ثمانم(ائة|ية)|تسعم(ائة|ية)|م[ئا]?[يئ][ةه])")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(
                    NumeralData::new(hundred_word(&s)?).with_grain(2),
                ))
            }),
        },
        Rule {
            name: "integer 101..999".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if [100.0,200.0,300.0,400.0,500.0,600.0,700.0,800.0,900.0].contains(&d.value))),
                regex("و"),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..100.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(م[ئا]?[يئ][ةه]|مئت(ان|ين)|مئات|[أا]لف(ان|ين)?|[آا]لاف|ملايين|مليون(ان|ين)?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                if matches!(s.as_str(), "مئة" | "مئه" | "مائة" | "مائه" | "مئات") {
                    return Some(TokenData::Numeral(
                        NumeralData::new(1e2).with_grain(2).with_multipliable(true),
                    ));
                }
                Some(TokenData::Numeral(power_word(&s)?))
            }),
        },
        Rule {
            name: "compose by multiplication".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), predicate(|td| matches!(td, TokenData::Numeral(d) if d.multipliable))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                if b.value > a.value {
                    let mut out = NumeralData::new(a.value * b.value);
                    if let Some(g) = b.grain {
                        out = out.with_grain(g);
                    }
                    Some(TokenData::Numeral(out))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "number dot number".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("فاصل[ةه]|فاصلة"), predicate(|td| matches!(td, TokenData::Numeral(d) if d.grain.is_none()))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + decimals_to_double(b))))
            }),
        },
        Rule {
            name: "numbers prefix with -, minus".to_string(),
            pattern: vec![regex("-"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
    ]
}
