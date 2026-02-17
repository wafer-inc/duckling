use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::PhoneNumberData;

fn normalize_decimal_digits(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            // Arabic-Indic
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
            // Extended Arabic-Indic
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

fn decode_escaped_arabic_indic_digits(s: &str) -> String {
    let mut out = String::new();
    let mut chunk = String::new();

    let flush_chunk = |chunk: &str, out: &mut String| {
        if chunk.is_empty() {
            return;
        }
        if chunk.len() % 4 == 0 {
            let mut ok = true;
            for group in chunk.as_bytes().chunks(4) {
                let g = std::str::from_utf8(group)
                    .ok()
                    .and_then(|x| x.parse::<u32>().ok());
                match g {
                    Some(cp @ 1632..=1641) => {
                        let d = (cp.saturating_sub(1632)) as u8;
                        out.push((d.saturating_add(b'0')) as char);
                    }
                    _ => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                return;
            }
        }
        out.push_str(chunk);
    };

    for ch in s.chars() {
        if ch.is_ascii_digit() {
            chunk.push(ch);
        } else {
            flush_chunk(&chunk, &mut out);
            chunk.clear();
            out.push(ch);
        }
    }
    flush_chunk(&chunk, &mut out);
    out
}

fn canonical_phone_digits(s: &str) -> String {
    let normalized = normalize_decimal_digits(s);
    let decoded = decode_escaped_arabic_indic_digits(&normalized);
    decoded.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn decimal_digit_count(s: &str) -> usize {
    canonical_phone_digits(s).len()
}

pub fn rules() -> Vec<Rule> {
    vec![
        // Comprehensive phone number rule ported from Haskell.
        // Matches: optional country code (+N or (+N)), number body with separators,
        // optional extension.
        // Since Rust regex doesn't support lookaheads, we validate digit count
        // in the production function (7-15 digits in the body).
        Rule {
            name: "phone number".to_string(),
            pattern: vec![regex(
                r"(?:\(?\+(\d{1,4})\)?[\s\-\.]*)?([\d(][\d()\s\-\.]{4,120}[\d)])(?:\s*(?:e?xt?\.?|x|فرعي)\s*(\d{1,40}))?",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let prefix = m.group(1);
                let body = m.group(2)?;
                let extension = m.group(3);

                // Count digits in the body
                let body_digits = decimal_digit_count(body);
                if !(7..=15).contains(&body_digits) {
                    return None;
                }

                // Build the cleaned phone number value (matching Haskell's cleanup)
                let mut value = String::new();
                if let Some(code) = prefix {
                    value.push_str(&format!("(+{}) ", canonical_phone_digits(code)));
                }
                value.push_str(&canonical_phone_digits(body));
                if let Some(ext) = extension {
                    value.push_str(&format!(" ext {}", canonical_phone_digits(ext)));
                }

                Some(TokenData::PhoneNumber(PhoneNumberData::new(&value)))
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::DimensionKind;

    #[test]
    fn test_phone_numbers() {
        let rules = rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for text in &[
            "650-701-8887",
            "(+1)650-701-8887",
            "+1 6507018887",
            "+33 1 46647998",
            "06 2070 2220",
            "4.8.6.6.8.2.7",
            "06354640807",
        ] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::PhoneNumber],
            );
            let found = entities
                .iter()
                .any(|e| matches!(&e.value, crate::types::DimensionValue::PhoneNumber(_)));
            assert!(
                found,
                "Expected phone number for '{}', got: {:?}",
                text, entities
            );
        }
    }

    #[test]
    fn test_no_phone_numbers() {
        let rules = rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for text in &["12345", "1234567890123456777777", "12345678901234567"] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::PhoneNumber],
            );
            let found = entities
                .iter()
                .any(|e| matches!(&e.value, crate::types::DimensionValue::PhoneNumber(_)));
            assert!(
                !found,
                "Expected NO phone number for '{}', got: {:?}",
                text, entities
            );
        }
    }
}
