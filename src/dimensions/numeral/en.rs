use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::helpers::*;
use super::NumeralData;

pub fn rules() -> Vec<Rule> {
    vec![
        // Integer: 0, 1, 2, ..., 9
        Rule {
            name: "integer (0..9)".to_string(),
            pattern: vec![regex(r#"(zero|naught|nought|nil|one|two|three|four|five|six|seven|eight|nine)"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val = match text.to_lowercase().as_str() {
                    "zero" | "naught" | "nought" | "nil" => 0.0,
                    "one" => 1.0,
                    "two" => 2.0,
                    "three" => 3.0,
                    "four" => 4.0,
                    "five" => 5.0,
                    "six" => 6.0,
                    "seven" => 7.0,
                    "eight" => 8.0,
                    "nine" => 9.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(val)))
            }),
        },
        // Integer: 10..19
        Rule {
            name: "integer (10..19)".to_string(),
            pattern: vec![regex(r#"(ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen)"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val = match text.to_lowercase().as_str() {
                    "ten" => 10.0,
                    "eleven" => 11.0,
                    "twelve" => 12.0,
                    "thirteen" => 13.0,
                    "fourteen" => 14.0,
                    "fifteen" => 15.0,
                    "sixteen" => 16.0,
                    "seventeen" => 17.0,
                    "eighteen" => 18.0,
                    "nineteen" => 19.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(val)))
            }),
        },
        // Integer: 20..90 (tens)
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex(r#"(twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety)"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val = match text.to_lowercase().as_str() {
                    "twenty" => 20.0,
                    "thirty" => 30.0,
                    "forty" => 40.0,
                    "fifty" => 50.0,
                    "sixty" => 60.0,
                    "seventy" => 70.0,
                    "eighty" => 80.0,
                    "ninety" => 90.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(val).with_multipliable(false)))
            }),
        },
        // Compose tens and units: twenty one, thirty-two, etc.
        Rule {
            name: "integer (21..99)".to_string(),
            pattern: vec![
                predicate(|t| {
                    if let TokenData::Numeral(d) = t {
                        let v = d.value;
                        v >= 20.0 && v <= 90.0 && v % 10.0 == 0.0
                    } else {
                        false
                    }
                }),
                regex(r#"-"#),
                predicate(|t| {
                    if let TokenData::Numeral(d) = t {
                        d.value >= 1.0 && d.value <= 9.0
                    } else {
                        false
                    }
                }),
            ],
            production: Box::new(|nodes| {
                let tens = numeral_data(&nodes[0].token_data)?.value;
                let units = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(tens + units)))
            }),
        },
        // Compose tens and units without separator: "twenty one" (space)
        Rule {
            name: "integer compose (tens + units)".to_string(),
            pattern: vec![
                predicate(|t| {
                    if let TokenData::Numeral(d) = t {
                        let v = d.value;
                        v >= 20.0 && v <= 90.0 && v % 10.0 == 0.0
                    } else {
                        false
                    }
                }),
                predicate(|t| {
                    if let TokenData::Numeral(d) = t {
                        d.value >= 1.0 && d.value <= 9.0
                    } else {
                        false
                    }
                }),
            ],
            production: Box::new(|nodes| {
                let tens = numeral_data(&nodes[0].token_data)?.value;
                let units = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(tens + units)))
            }),
        },
        // Numeric digits
        Rule {
            name: "integer (numeric)".to_string(),
            pattern: vec![regex(r#"(\d{1,18})"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val: f64 = text.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(val)))
            }),
        },
        // Decimal numbers: 1.5, 3.14, .5
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex(r#"(\d*\.\d+)"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val: f64 = text.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(val)))
            }),
        },
        // Negative numbers
        Rule {
            name: "negative number".to_string(),
            pattern: vec![
                regex(r#"(-|minus|negative)\s?"#),
                dim(DimensionKind::Numeral),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[1].token_data)?;
                Some(TokenData::Numeral(NumeralData::new(-data.value)))
            }),
        },
        // Hundred
        Rule {
            name: "hundred".to_string(),
            pattern: vec![regex(r#"(hundred|hundreds)"#)],
            production: Box::new(|_nodes| {
                Some(TokenData::Numeral(
                    NumeralData::new(100.0)
                        .with_grain(2)
                        .with_multipliable(true),
                ))
            }),
        },
        // Thousand
        Rule {
            name: "thousand".to_string(),
            pattern: vec![regex(r#"(thousand|thousands)"#)],
            production: Box::new(|_nodes| {
                Some(TokenData::Numeral(
                    NumeralData::new(1000.0)
                        .with_grain(3)
                        .with_multipliable(true),
                ))
            }),
        },
        // Million
        Rule {
            name: "million".to_string(),
            pattern: vec![regex(r#"(million|millions)"#)],
            production: Box::new(|_nodes| {
                Some(TokenData::Numeral(
                    NumeralData::new(1_000_000.0)
                        .with_grain(6)
                        .with_multipliable(true),
                ))
            }),
        },
        // Billion
        Rule {
            name: "billion".to_string(),
            pattern: vec![regex(r#"(billion|billions)"#)],
            production: Box::new(|_nodes| {
                Some(TokenData::Numeral(
                    NumeralData::new(1_000_000_000.0)
                        .with_grain(9)
                        .with_multipliable(true),
                ))
            }),
        },
        // Number with K/M/G suffix: 100K, 5M
        Rule {
            name: "number suffixes (K, M, G)".to_string(),
            pattern: vec![regex(r#"(\d+(?:\.\d+)?)\s*(k|m|g|b)"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let caps = regex::Regex::new(r"(?i)(\d+(?:\.\d+)?)\s*(k|m|g|b)")
                    .ok()?
                    .captures(text)?;
                let num: f64 = caps.get(1)?.as_str().parse().ok()?;
                let suffix = caps.get(2)?.as_str().to_lowercase();
                let multiplier = match suffix.as_str() {
                    "k" => 1_000.0,
                    "m" => 1_000_000.0,
                    "g" | "b" => 1_000_000_000.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(num * multiplier)))
            }),
        },
        // Compose with multiplier: "five hundred", "two thousand"
        Rule {
            name: "compose (multiplier)".to_string(),
            pattern: vec![
                predicate(|t| {
                    if let TokenData::Numeral(d) = t {
                        d.value >= 1.0 && d.value <= 99.0
                    } else {
                        false
                    }
                }),
                predicate(is_multipliable),
            ],
            production: Box::new(|nodes| {
                let base = numeral_data(&nodes[0].token_data)?.value;
                let mult = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(
                    NumeralData::new(base * mult).with_grain(
                        numeral_data(&nodes[1].token_data)?
                            .grain
                            .unwrap_or(0),
                    ),
                ))
            }),
        },
        // Number with comma: 1,000 or 1,000,000
        Rule {
            name: "number with commas".to_string(),
            pattern: vec![regex(r#"(\d{1,3}(?:,\d{3})+)"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let cleaned = text.replace(',', "");
                let val: f64 = cleaned.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(val)))
            }),
        },
        // a pair, a couple
        Rule {
            name: "a pair / a couple".to_string(),
            pattern: vec![regex(r#"(a\s+)?(pair|couple)(\s+of)?"#)],
            production: Box::new(|_nodes| {
                Some(TokenData::Numeral(NumeralData::new(2.0)))
            }),
        },
        // a dozen
        Rule {
            name: "a dozen".to_string(),
            pattern: vec![regex(r#"(a\s+)?dozen"#)],
            production: Box::new(|_nodes| {
                Some(TokenData::Numeral(NumeralData::new(12.0)))
            }),
        },
        // Sum composite: "five hundred twenty three" => 523
        Rule {
            name: "sum composite".to_string(),
            pattern: vec![
                predicate(|t| {
                    if let TokenData::Numeral(d) = t {
                        d.value >= 100.0 && d.grain.is_some()
                    } else {
                        false
                    }
                }),
                predicate(|t| {
                    if let TokenData::Numeral(d) = t {
                        d.value >= 1.0 && d.value <= 99.0
                    } else {
                        false
                    }
                }),
            ],
            production: Box::new(|nodes| {
                let big = numeral_data(&nodes[0].token_data)?.value;
                let small = numeral_data(&nodes[1].token_data)?.value;
                if big > small {
                    Some(TokenData::Numeral(NumeralData::new(big + small)))
                } else {
                    None
                }
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::testing::{check_corpus, Corpus};
    use crate::types::DimensionKind;
    use chrono::Utc;

    fn build_rules() -> Vec<Rule> {
        rules()
    }

    #[test]
    fn test_numeral_corpus() {
        let context = Context {
            reference_time: Utc::now(),
            locale: crate::locale::Locale::default(),
        };
        let mut corpus = Corpus::new(context);

        corpus.add(vec!["zero", "Zero", "ZERO"], |e| {
            e.dim == "number"
                && e.value
                    .value
                    .get("value")
                    .and_then(|v| v.as_f64())
                    .map(|v| v == 0.0)
                    .unwrap_or(false)
        });

        corpus.add(vec!["one", "One"], |e| {
            e.dim == "number"
                && e.value
                    .value
                    .get("value")
                    .and_then(|v| v.as_f64())
                    .map(|v| v == 1.0)
                    .unwrap_or(false)
        });

        corpus.add(vec!["fifteen", "Fifteen"], |e| {
            e.dim == "number"
                && e.value
                    .value
                    .get("value")
                    .and_then(|v| v.as_f64())
                    .map(|v| v == 15.0)
                    .unwrap_or(false)
        });

        let rules = build_rules();
        let failures = check_corpus(&corpus, &rules, &[DimensionKind::Numeral]);
        assert!(failures.is_empty(), "Failures: {:?}", failures);
    }

    #[test]
    fn test_thirty_three() {
        let rules = build_rules();
        let options = Options {
            with_latent: false,
        };
        let context = Context::default();
        let entities = engine::parse_and_resolve(
            "thirty three",
            &rules,
            &context,
            &options,
            &[DimensionKind::Numeral],
        );
        let found = entities.iter().any(|e| {
            e.dim == "number"
                && e.value
                    .value
                    .get("value")
                    .and_then(|v| v.as_f64())
                    .map(|v| (v - 33.0).abs() < 0.01)
                    .unwrap_or(false)
        });
        assert!(found, "Expected to find 33, got: {:?}", entities);
    }

    #[test]
    fn test_numeric_integers() {
        let rules = build_rules();
        let options = Options {
            with_latent: false,
        };
        let context = Context::default();

        let entities =
            engine::parse_and_resolve("42", &rules, &context, &options, &[DimensionKind::Numeral]);
        let found = entities.iter().any(|e| {
            e.value
                .value
                .get("value")
                .and_then(|v| v.as_f64())
                .map(|v| (v - 42.0).abs() < 0.01)
                .unwrap_or(false)
        });
        assert!(found, "Expected 42, got: {:?}", entities);
    }

    #[test]
    fn test_100k() {
        let rules = build_rules();
        let options = Options {
            with_latent: false,
        };
        let context = Context::default();

        let entities = engine::parse_and_resolve(
            "100K",
            &rules,
            &context,
            &options,
            &[DimensionKind::Numeral],
        );
        let found = entities.iter().any(|e| {
            e.value
                .value
                .get("value")
                .and_then(|v| v.as_f64())
                .map(|v| (v - 100_000.0).abs() < 0.01)
                .unwrap_or(false)
        });
        assert!(found, "Expected 100000, got: {:?}", entities);
    }

    #[test]
    fn test_five_hundred() {
        let rules = build_rules();
        let options = Options {
            with_latent: false,
        };
        let context = Context::default();

        let entities = engine::parse_and_resolve(
            "five hundred",
            &rules,
            &context,
            &options,
            &[DimensionKind::Numeral],
        );
        let found = entities.iter().any(|e| {
            e.value
                .value
                .get("value")
                .and_then(|v| v.as_f64())
                .map(|v| (v - 500.0).abs() < 0.01)
                .unwrap_or(false)
        });
        assert!(found, "Expected 500, got: {:?}", entities);
    }
}
