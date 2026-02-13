use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn is_positive(td: &TokenData) -> bool { matches!(td, TokenData::Numeral(d) if d.value >= 0.0) }
fn has_grain(td: &TokenData) -> bool { matches!(td, TokenData::Numeral(d) if d.grain.is_some()) }
fn is_multipliable(td: &TokenData) -> bool { matches!(td, TokenData::Numeral(d) if d.multipliable) }
fn number_between(low:f64, up:f64)->impl Fn(&TokenData)->bool{ move |td| matches!(td, TokenData::Numeral(d) if d.value>=low && d.value<up)}

fn zero_to_nineteen(s:&str)->Option<f64>{
    match s {
        "inget"|"ingen"|"noll"=>Some(0.0),"en"|"ett"=>Some(1.0),"två"=>Some(2.0),"tre"=>Some(3.0),"fyra"=>Some(4.0),"fem"=>Some(5.0),"sex"=>Some(6.0),"sju"=>Some(7.0),"åtta"=>Some(8.0),"nio"=>Some(9.0),"tio"=>Some(10.0),"elva"=>Some(11.0),"tolv"=>Some(12.0),"tretton"=>Some(13.0),"fjorton"=>Some(14.0),"femton"=>Some(15.0),"sexton"=>Some(16.0),"sjutton"=>Some(17.0),"arton"=>Some(18.0),"nitton"=>Some(19.0), _=>None
    }
}
fn tens(s:&str)->Option<f64>{ match s {"tjugo"=>Some(20.0),"trettio"=>Some(30.0),"fyrtio"=>Some(40.0),"femtio"=>Some(50.0),"sextio"=>Some(60.0),"sjuttio"=>Some(70.0),"åttio"=>Some(80.0),"nittio"=>Some(90.0),_=>None}}

pub fn rules()->Vec<Rule>{
    vec![
        Rule{name:"intersect (with and)".to_string(),pattern:vec![predicate(has_grain),regex("och"),predicate(|td| !is_multipliable(td)&&is_positive(td))],production:Box::new(|n|{let a=numeral_data(&n[0].token_data)?;let b=numeral_data(&n[2].token_data)?;let g=a.grain?;if 10f64.powi(g as i32)>b.value{Some(TokenData::Numeral(NumeralData::new(a.value+b.value)))}else{None}})},
        Rule{name:"numbers prefix with -, negative or minus".to_string(),pattern:vec![regex("-|minus|negativ"),predicate(is_positive)],production:Box::new(|n|{let v=numeral_data(&n[1].token_data)?.value;Some(TokenData::Numeral(NumeralData::new(-v)))})},
        Rule{name:"few".to_string(),pattern:vec![regex("(några )?få")],production:Box::new(|_|Some(TokenData::Numeral(NumeralData::new(3.0))))},
        Rule{name:"decimal with thousands separator".to_string(),pattern:vec![regex("(\\d+(\\.\\d\\d\\d)+\\,\\d+)")],production:Box::new(|n|{let t=match &n[0].token_data{TokenData::RegexMatch(m)=>m.group(1)?,_=>return None};Some(TokenData::Numeral(NumeralData::new(t.replace('.',"").replace(',',".").parse().ok()?)))})},
        Rule{name:"decimal number".to_string(),pattern:vec![regex("(\\d*,\\d+)")],production:Box::new(|n|{let t=match &n[0].token_data{TokenData::RegexMatch(m)=>m.group(1)?,_=>return None};Some(TokenData::Numeral(NumeralData::new(t.replace(',',".").parse().ok()?)))})},
        Rule{name:"integer 21..99".to_string(),pattern:vec![predicate(|td|matches!(td,TokenData::Numeral(x) if [70.0,20.0,60.0,50.0,40.0,90.0,30.0,80.0].contains(&x.value))),predicate(number_between(1.0,10.0))],production:Box::new(|n|{let a=numeral_data(&n[0].token_data)?.value;let b=numeral_data(&n[1].token_data)?.value;Some(TokenData::Numeral(NumeralData::new(a+b)))})},
        Rule{name:"single".to_string(),pattern:vec![regex("enkel")],production:Box::new(|_|Some(TokenData::Numeral(NumeralData::new(1.0).with_grain(1))))},
        Rule{name:"intersect".to_string(),pattern:vec![predicate(has_grain),predicate(|td| !is_multipliable(td)&&is_positive(td))],production:Box::new(|n|{let a=numeral_data(&n[0].token_data)?;let b=numeral_data(&n[1].token_data)?;let g=a.grain?;if 10f64.powi(g as i32)>b.value{Some(TokenData::Numeral(NumeralData::new(a.value+b.value)))}else{None}})},
        Rule{name:"compose by multiplication".to_string(),pattern:vec![predicate(|td|matches!(td,TokenData::Numeral(_))),predicate(is_multipliable)],production:Box::new(|n|{let a=numeral_data(&n[0].token_data)?;let b=numeral_data(&n[1].token_data)?;if b.grain.is_none()||(b.grain.is_some()&&b.value>a.value){let mut o=NumeralData::new(a.value*b.value);if let Some(g)=b.grain{o=o.with_grain(g);}Some(TokenData::Numeral(o))}else{None}})},
        Rule{name:"numbers suffixes (K, M, G)".to_string(),pattern:vec![predicate(|td|matches!(td,TokenData::Numeral(_))),regex("([kmg])")],production:Box::new(|n|{let v=numeral_data(&n[0].token_data)?.value;let s=match &n[1].token_data{TokenData::RegexMatch(m)=>m.group(1)?.to_lowercase(),_=>return None};let o=match s.as_str(){"k"=>v*1e3,"m"=>v*1e6,"g"=>v*1e9,_=>return None};Some(TokenData::Numeral(NumeralData::new(o)))})},
        Rule{name:"powers of tens".to_string(),pattern:vec![regex("(hundra?|tusen?|miljon(er)?)")],production:Box::new(|n|{let s=match &n[0].token_data{TokenData::RegexMatch(m)=>m.group(1)?.to_lowercase(),_=>return None};let o=match s.as_str(){"hundr"|"hundra"=>NumeralData::new(1e2).with_grain(2).with_multipliable(true),"tuse"|"tusen"=>NumeralData::new(1e3).with_grain(3).with_multipliable(true),"miljon"|"miljoner"=>NumeralData::new(1e6).with_grain(6).with_multipliable(true),_=>return None};Some(TokenData::Numeral(o))})},
        Rule{name:"couple, a pair".to_string(),pattern:vec![regex("ett par")],production:Box::new(|_|Some(TokenData::Numeral(NumeralData::new(2.0))))},
        Rule{name:"dozen".to_string(),pattern:vec![regex("dussin")],production:Box::new(|_|Some(TokenData::Numeral(NumeralData::new(12.0).with_grain(1).with_multipliable(true))))},
        Rule{name:"integer (0..19)".to_string(),pattern:vec![regex("(inget|ingen|noll|en|ett|två|tretton|tre|fyra|femton|fem|sexton|sex|sjutton|sju|åtta|nio|tio|elva|tolv|fjorton|arton|nitton)")],production:Box::new(|n|{let s=match &n[0].token_data{TokenData::RegexMatch(m)=>m.group(1)?.to_lowercase(),_=>return None};Some(TokenData::Numeral(NumeralData::new(zero_to_nineteen(&s)?)))})},
        Rule{name:"integer (20..90)".to_string(),pattern:vec![regex("(tjugo|trettio|fyrtio|femtio|sextio|sjuttio|åttio|nittio)")],production:Box::new(|n|{let s=match &n[0].token_data{TokenData::RegexMatch(m)=>m.group(1)?.to_lowercase(),_=>return None};Some(TokenData::Numeral(NumeralData::new(tens(&s)?)))})},
        Rule{name:"number dot number".to_string(),pattern:vec![predicate(|td|matches!(td,TokenData::Numeral(_))),regex("komma"),predicate(|td|matches!(td,TokenData::Numeral(x) if x.grain.is_none()))],production:Box::new(|n|{let a=numeral_data(&n[0].token_data)?.value;let b=numeral_data(&n[2].token_data)?.value;let mut m=1.0;while b>=m{m*=10.0;}Some(TokenData::Numeral(NumeralData::new(a+b/m)))})},
        Rule{name:"integer with thousands separator .".to_string(),pattern:vec![regex("(\\d{1,3}(\\.\\d\\d\\d){1,5})")],production:Box::new(|n|{let t=match &n[0].token_data{TokenData::RegexMatch(m)=>m.group(1)?,_=>return None};Some(TokenData::Numeral(NumeralData::new(t.replace('.',"").parse().ok()?)))})},
    ]
}
