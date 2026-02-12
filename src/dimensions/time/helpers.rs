use super::TimeData;
use crate::types::TokenData;

pub fn time_data(token: &TokenData) -> Option<&TimeData> {
    match token {
        TokenData::Time(data) => Some(data),
        _ => None,
    }
}

pub fn is_time(token: &TokenData) -> bool {
    matches!(token, TokenData::Time(_))
}
