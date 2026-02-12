use chrono::{DateTime, Utc};
use crate::dimensions;
use crate::locale::Locale;
use crate::types::{DimensionValue, Entity, Node, TokenData};

/// Context for resolving parsed tokens into structured values.
#[derive(Debug, Clone)]
pub struct Context {
    pub reference_time: DateTime<Utc>,
    pub locale: Locale,
    /// Context timezone offset in minutes from UTC (e.g., -120 for UTC-2)
    pub timezone_offset_minutes: i32,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            reference_time: Utc::now(),
            locale: Locale::default(),
            timezone_offset_minutes: 0,
        }
    }
}

/// Options for controlling parsing behavior.
#[derive(Debug, Clone, Default)]
pub struct Options {
    pub with_latent: bool,
}

/// Resolve a node into a structured entity.
pub fn resolve(node: &Node, context: &Context, options: &Options, text: &str) -> Option<Entity> {
    let body = text[node.range.start..node.range.end].to_string();
    let resolved = resolve_token(&node.token_data, context, options)?;

    Some(Entity {
        body,
        start: node.range.start,
        end: node.range.end,
        value: resolved,
        latent: None,
    })
}

fn resolve_token(token: &TokenData, context: &Context, options: &Options) -> Option<DimensionValue> {
    match token {
        TokenData::Numeral(data) => Some(dimensions::numeral::resolve(data)),
        TokenData::Ordinal(data) => Some(dimensions::ordinal::resolve(data)),
        TokenData::Temperature(data) => dimensions::temperature::resolve(data),
        TokenData::Distance(data) => dimensions::distance::resolve(data),
        TokenData::Volume(data) => dimensions::volume::resolve(data),
        TokenData::Quantity(data) => dimensions::quantity::resolve(data),
        TokenData::AmountOfMoney(data) => {
            dimensions::amount_of_money::resolve(data, options.with_latent)
        }
        TokenData::Email(data) => Some(dimensions::email::resolve(data)),
        TokenData::PhoneNumber(data) => Some(dimensions::phone_number::resolve(data)),
        TokenData::Url(data) => Some(dimensions::url::resolve(data)),
        TokenData::CreditCardNumber(data) => {
            Some(dimensions::credit_card_number::resolve(data))
        }
        TokenData::TimeGrain(grain) => Some(dimensions::time_grain::resolve(grain)),
        TokenData::Duration(data) => Some(dimensions::duration::resolve(data)),
        TokenData::Time(data) => {
            dimensions::time::resolve(data, context, options.with_latent)
        }
        TokenData::RegexMatch(_) => None,
    }
}
