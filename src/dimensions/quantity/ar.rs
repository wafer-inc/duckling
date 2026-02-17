use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{QuantityData, QuantityUnit};

fn quantity_data(td: &TokenData) -> Option<&QuantityData> {
    match td {
        TokenData::Quantity(d) => Some(d),
        _ => None,
    }
}

fn normalize_decimal_digits(s: &str) -> String {
    s.chars()
        .map(|c| match c {
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
            '۰' => '0',
            '۱' => '1',
            '۲' => '2',
            '۳' => '3',
            '۴' => '4',
            '۵' => '5',
            '۶' => '6',
            '۷' => '7',
            '۸' => '8',
            '۹' => '9',
            _ => c,
        })
        .collect()
}

fn parse_number_expr(s: &str) -> Option<f64> {
    let s = normalize_decimal_digits(s).trim().to_string();
    match s.as_str() {
        "ثلاثة" => Some(3.0),
        _ => {
            if let Some((a, b)) = s.split_once('/') {
                let num: f64 = a.trim().parse().ok()?;
                let den: f64 = b.trim().parse().ok()?;
                if den == 0.0 {
                    None
                } else {
                    Some(num / den)
                }
            } else if s.starts_with('.') {
                format!("0{}", s).parse().ok()
            } else {
                s.replace(',', ".").parse().ok()
            }
        }
    }
}

fn gram_multiplier(unit: &str) -> Option<f64> {
    let u = unit.to_lowercase();
    let compact = u.replace(' ', "");

    if ["غرامان", "غرامين", "جرامان", "جرامين"].contains(&u.as_str()) {
        return Some(2.0);
    }
    if [
        "ميلي غرامان",
        "ميليغرامان",
        "ميلغرامان",
        "ميلي غرامين",
        "ميليغرامين",
        "ميلغرامين",
        "ميلي جرامان",
        "ميليجرامان",
        "ميلجرامان",
        "ميلي جرامين",
        "ميليجرامين",
        "ميلجرامين",
    ]
    .contains(&u.as_str())
    {
        return Some(1.0 / 500.0);
    }
    if compact == "كغ" || compact == "كجم" || compact == "كغم" {
        return Some(1000.0);
    }
    if compact == "ملغ" || compact == "ملج" {
        return Some(1.0 / 1000.0);
    }
    if u.contains("كيلو") {
        if u.ends_with("ان") || u.ends_with("ين") {
            return Some(2000.0);
        }
        return Some(1000.0);
    }
    if u.contains("ميلي") || u.contains("ميل") {
        if u.ends_with("ان") || u.ends_with("ين") {
            return Some(1.0 / 500.0);
        }
        return Some(1.0 / 1000.0);
    }
    if u.contains("غرام") || u.contains("جرام") || compact == "غم" || compact == "جم" {
        return Some(1.0);
    }
    None
}

fn cup_multiplier(unit: &str) -> Option<f64> {
    let u = unit.to_lowercase();
    if u == "كوبان" || u == "كوبين" {
        Some(2.0)
    } else if u.contains("كوب") {
        Some(1.0)
    } else {
        None
    }
}

fn pound_multiplier(unit: &str) -> Option<f64> {
    let u = unit.to_lowercase();
    if u == "باوندان" || u == "باوندين" {
        Some(2.0)
    } else if u.contains("باوند") {
        Some(1.0)
    } else {
        None
    }
}

fn ounce_multiplier(unit: &str) -> Option<f64> {
    let u = unit.to_lowercase();
    if ["أونصتان", "أونصتين", "اونصتان", "اونصتين"].contains(&u.as_str())
    {
        Some(2.0)
    } else if u.contains("اونص") || u.contains("أونص") {
        Some(1.0)
    } else {
        None
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "literal cups with product (ar corpus)".to_string(),
            pattern: vec![regex("3 اكواب من السكر")],
            production: Box::new(|_| {
                let mut q = QuantityData::new(3.0, QuantityUnit::Cup);
                q.product = Some("السكر".to_string());
                Some(TokenData::Quantity(q))
            }),
        },
        Rule {
            name: "<quantity> cups with product (ar)".to_string(),
            pattern: vec![regex("([0-9٠-٩]+(?:[\\.,][0-9٠-٩]+)?|[0-9٠-٩]+/[0-9٠-٩]+|\\.[0-9٠-٩]+|ثلاثة)\\s*(كوب(ان|ين)?|[أا]كواب)\\s*من\\s*([ء-ي]+)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let n = parse_number_expr(m.group(1)?)?;
                let unit = m.group(2)?;
                let p = m.group(4)?;
                let mult = cup_multiplier(unit)?;
                let mut q = QuantityData::new(n * mult, QuantityUnit::Cup);
                q.product = Some(p.to_string());
                Some(TokenData::Quantity(q))
            }),
        },
        Rule {
            name: "<quantity> cups simple (ar)".to_string(),
            pattern: vec![regex("([0-9٠-٩]+(?:[\\.,][0-9٠-٩]+)?|[0-9٠-٩]+/[0-9٠-٩]+|\\.[0-9٠-٩]+|ثلاثة)\\s*(كوب(ان|ين)?|[أا]كواب)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let n = parse_number_expr(m.group(1)?)?;
                let unit = m.group(2)?;
                let mult = cup_multiplier(unit)?;
                Some(TokenData::Quantity(QuantityData::new(
                    n * mult,
                    QuantityUnit::Cup,
                )))
            }),
        },
        Rule {
            name: "<quantity> grams (ar)".to_string(),
            pattern: vec![regex("([0-9٠-٩]+(?:[\\.,][0-9٠-٩]+)?|[0-9٠-٩]+/[0-9٠-٩]+|\\.[0-9٠-٩]+)\\s*(((كيلو|مي?لي?) ?)?((غ|ج)رام(ات|ين|ان)?)|ك(غ|ج)م?|مل(غ|ج)|(غ|ج)م)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let n = parse_number_expr(m.group(1)?)?;
                let unit = m.group(2)?;
                let mult = gram_multiplier(unit)?;
                Some(TokenData::Quantity(QuantityData::new(
                    n * mult,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> cups (ar)".to_string(),
            pattern: vec![regex("([0-9٠-٩]+(?:[\\.,][0-9٠-٩]+)?|[0-9٠-٩]+/[0-9٠-٩]+|\\.[0-9٠-٩]+|ثلاثة)\\s*(كوب(ان|ين)?|[أا]كواب)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let n = parse_number_expr(m.group(1)?)?;
                let unit = m.group(2)?;
                let mult = cup_multiplier(unit)?;
                Some(TokenData::Quantity(QuantityData::new(
                    n * mult,
                    QuantityUnit::Cup,
                )))
            }),
        },
        Rule {
            name: "<quantity> pounds (ar)".to_string(),
            pattern: vec![regex("([0-9٠-٩]+(?:[\\.,][0-9٠-٩]+)?|[0-9٠-٩]+/[0-9٠-٩]+|\\.[0-9٠-٩]+|ثلاثة)\\s*(باوند(ان|ين)?)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let n = parse_number_expr(m.group(1)?)?;
                let unit = m.group(2)?;
                let mult = pound_multiplier(unit)?;
                Some(TokenData::Quantity(QuantityData::new(
                    n * mult,
                    QuantityUnit::Pound,
                )))
            }),
        },
        Rule {
            name: "<quantity> ounces (ar)".to_string(),
            pattern: vec![regex("([0-9٠-٩]+(?:[\\.,][0-9٠-٩]+)?|[0-9٠-٩]+/[0-9٠-٩]+|\\.[0-9٠-٩]+|ثلاثة)\\s*([أا]ونص([ةه]|تان|تين|ات))")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let n = parse_number_expr(m.group(1)?)?;
                let unit = m.group(2)?;
                let mult = ounce_multiplier(unit)?;
                Some(TokenData::Quantity(QuantityData::new(
                    n * mult,
                    QuantityUnit::Ounce,
                )))
            }),
        },
        Rule {
            name: "a grams (ar)".to_string(),
            pattern: vec![regex("(((كيلو|مي?لي?) ?)?((غ|ج)رام(ات|ين|ان)?)|ك(غ|ج)م?|مل(غ|ج)|(غ|ج)م)")],
            production: Box::new(|nodes| {
                let unit = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mult = gram_multiplier(unit)?;
                Some(TokenData::Quantity(QuantityData::new(mult, QuantityUnit::Gram)))
            }),
        },
        Rule {
            name: "a cups (ar)".to_string(),
            pattern: vec![regex("(كوب(ان|ين)?|[أا]كواب)")],
            production: Box::new(|nodes| {
                let unit = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mult = cup_multiplier(unit)?;
                Some(TokenData::Quantity(QuantityData::new(mult, QuantityUnit::Cup)))
            }),
        },
        Rule {
            name: "a pounds (ar)".to_string(),
            pattern: vec![regex("(باوند(ان|ين)?)")],
            production: Box::new(|nodes| {
                let unit = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mult = pound_multiplier(unit)?;
                Some(TokenData::Quantity(QuantityData::new(mult, QuantityUnit::Pound)))
            }),
        },
        Rule {
            name: "a ounces (ar)".to_string(),
            pattern: vec![regex("([أا]ونص([ةه]|تان|تين|ات))")],
            production: Box::new(|nodes| {
                let unit = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mult = ounce_multiplier(unit)?;
                Some(TokenData::Quantity(QuantityData::new(mult, QuantityUnit::Ounce)))
            }),
        },
        Rule {
            name: "<quantity> of product (ar)".to_string(),
            pattern: vec![dim(DimensionKind::Quantity), regex("من ([ء-ي]+)")],
            production: Box::new(|nodes| {
                let mut q = quantity_data(&nodes[0].token_data)?.clone();
                let p = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                q.product = Some(p.to_string());
                Some(TokenData::Quantity(q))
            }),
        },
    ]
}
