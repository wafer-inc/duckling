use crate::dimensions::numeral::helpers::numeral_data;
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

fn is_natural(td: &TokenData) -> bool { matches!(td, TokenData::Numeral(d) if d.value > 0.0 && d.value.fract()==0.0) }

pub fn rules()->Vec<Rule>{vec![
    Rule{name:"half an hour".to_string(),pattern:vec![regex("(1/2|en halv) timme")],production:Box::new(|_|Some(TokenData::Duration(DurationData::new(30,Grain::Minute))))},
    Rule{name:"<integer> more <unit-of-duration>".to_string(),pattern:vec![predicate(is_natural),dim(DimensionKind::TimeGrain),regex("fler|mer")],production:Box::new(|n|{let v=numeral_data(&n[0].token_data)?.value as i64;let g=match &n[1].token_data{TokenData::TimeGrain(g)=>*g,_=>return None};Some(TokenData::Duration(DurationData::new(v,g)))})},
    Rule{name:"number.number hours".to_string(),pattern:vec![regex("(\\d+)\\,(\\d+)"),regex("timm(e|ar)?")],production:Box::new(|n|{let m=match &n[0].token_data{TokenData::RegexMatch(m)=>m,_=>return None};let hh:i64=m.group(1)?.parse().ok()?;let frac=m.group(2)?;let num:i64=frac.parse().ok()?;let den:i64=10_i64.pow(frac.len() as u32);Some(TokenData::Duration(DurationData::new(60_i64.checked_mul(hh)?.checked_add(num.checked_mul(60)?.checked_div(den)?)?, Grain::Minute)))})},
    Rule{name:"<integer> and an half hours".to_string(),pattern:vec![predicate(is_natural),regex("och (en )?halv timme?")],production:Box::new(|n|{let v=numeral_data(&n[0].token_data)?.value as i64;Some(TokenData::Duration(DurationData::new(60_i64.checked_mul(v)?.checked_add(30)?,Grain::Minute)))})},
    Rule{name:"a <unit-of-duration>".to_string(),pattern:vec![regex("en|ett?"),dim(DimensionKind::TimeGrain)],production:Box::new(|n|{let g=match &n[1].token_data{TokenData::TimeGrain(g)=>*g,_=>return None};Some(TokenData::Duration(DurationData::new(1,g)))})},
    Rule{name:"about <duration>".to_string(),pattern:vec![regex("(omkring|cirka|ca\\.?|c:a|runt|ungefär)"),dim(DimensionKind::Duration)],production:Box::new(|n|Some(n[1].token_data.clone()))},
    Rule{name:"exactly <duration>".to_string(),pattern:vec![regex("(precis|exakt)"),dim(DimensionKind::Duration)],production:Box::new(|n|Some(n[1].token_data.clone()))},
]}
