use chrono::{DateTime, Utc};
use crate::dimensions;
use crate::locale::Locale;
use crate::types::{Entity, Node, ResolvedValue, TokenData};

/// Context for resolving parsed tokens into structured values.
#[derive(Debug, Clone)]
pub struct Context {
    pub reference_time: DateTime<Utc>,
    pub locale: Locale,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            reference_time: Utc::now(),
            locale: Locale::default(),
        }
    }
}

/// Options for controlling parsing behavior.
#[derive(Debug, Clone, Default)]
pub struct Options {
    pub with_latent: bool,
}

/// Resolve a node into a structured entity.
pub fn resolve(node: &Node, context: &Context, _options: &Options, text: &str) -> Option<Entity> {
    let body = text[node.range.start..node.range.end].to_string();
    let resolved = resolve_token(&node.token_data, context)?;

    Some(Entity {
        body,
        start: node.range.start,
        end: node.range.end,
        dim: node
            .token_data
            .dimension_kind()
            .map(|d| d.to_string())
            .unwrap_or_default(),
        value: resolved,
        latent: None,
    })
}

fn resolve_token(token: &TokenData, context: &Context) -> Option<ResolvedValue> {
    match token {
        TokenData::Numeral(data) => Some(dimensions::numeral::resolve(data)),
        TokenData::Ordinal(data) => Some(dimensions::ordinal::resolve(data)),
        TokenData::Temperature(data) => dimensions::temperature::resolve(data),
        TokenData::Distance(data) => Some(dimensions::distance::resolve(data)),
        TokenData::Volume(data) => Some(dimensions::volume::resolve(data)),
        TokenData::Quantity(data) => Some(dimensions::quantity::resolve(data)),
        TokenData::AmountOfMoney(data) => Some(dimensions::amount_of_money::resolve(data)),
        TokenData::Email(data) => Some(dimensions::email::resolve(data)),
        TokenData::PhoneNumber(data) => Some(dimensions::phone_number::resolve(data)),
        TokenData::Url(data) => Some(dimensions::url::resolve(data)),
        TokenData::CreditCardNumber(data) => {
            Some(dimensions::credit_card_number::resolve(data))
        }
        TokenData::TimeGrain(grain) => Some(dimensions::time_grain::resolve(grain)),
        TokenData::Duration(data) => Some(dimensions::duration::resolve(data)),
        TokenData::Time(data) => Some(dimensions::time::resolve(data, context)),
        TokenData::RegexMatch(_) => None,
    }
}
