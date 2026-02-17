use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn ord(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "första" | "förste" => Some(1), "andra" | "andre" => Some(2), "tredje" => Some(3), "fjärde" => Some(4), "femte" => Some(5), "sjätte" => Some(6), "sjunde" => Some(7), "åttonde" => Some(8), "nionde" => Some(9), "tionde" => Some(10), "elfte" => Some(11), "tolfte" => Some(12), "trettonde" => Some(13), "fjortonde" => Some(14), "femtonde" => Some(15), "sextonde" => Some(16), "sjuttonde" => Some(17), "artonde" => Some(18), "nittonde" => Some(19), "tjugonde" => Some(20), "trettionde" => Some(30), "fyrtionde" => Some(40), "femtionde" => Some(50), "sextionde" => Some(60), "sjuttionde" => Some(70), "åttionde" => Some(80), "nittionde" => Some(90), _ => None
    }
}
fn card(s: &str)->Option<i64>{match s.to_lowercase().as_str(){"tjugo"=>Some(20),"trettio"=>Some(30),"fyrtio"=>Some(40),"femtio"=>Some(50),"sextio"=>Some(60),"sjuttio"=>Some(70),"åttio"=>Some(80),"nittio"=>Some(90),_=>None}}

pub fn rules()->Vec<Rule>{vec![
    Rule{name:"ordinals (first..twentieth,thirtieth,...)".to_string(),pattern:vec![regex("(första|förste|andra|andre|tredje|fjärde|femte|sjätte|sjunde|åttonde|nionde|tionde|elfte|tolfte|trettionde|fjortonde|femtonde|sextonde|sjuttonde|artonde|nittonde|tjugonde|trettionde|fyrtionde|femtonde|sextionde|sjuttionde|åttionde|nittionde)")],production:Box::new(|n|{let s=match &n[0].token_data{TokenData::RegexMatch(m)=>m.group(1)?,_=>return None};Some(TokenData::Ordinal(OrdinalData::new(ord(s)?)))})},
    Rule{name:"ordinals (composite, e.g., eighty-seven)".to_string(),pattern:vec![regex("(tjugo|trettio|fyrtio|femtio|sextio|sjuttio|åttio|nittio)(första|förste|andra|andre|tredje|fjärde|femte|sjätte|sjunde|åttonde|nionde)")],production:Box::new(|n|{let m=match &n[0].token_data{TokenData::RegexMatch(m)=>m,_=>return None};Some(TokenData::Ordinal(OrdinalData::new(card(m.group(1)?)?.checked_add(ord(m.group(2)?)?)?)))})},
    Rule{name:"ordinal (digits)".to_string(),pattern:vec![regex("0*(\\d+):?(a|e)")],production:Box::new(|n|{let v: i64=match &n[0].token_data{TokenData::RegexMatch(m)=>m.group(1)?.parse().ok()?,_=>return None};Some(TokenData::Ordinal(OrdinalData::new(v)))})},
]}
