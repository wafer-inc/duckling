use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_ordinal(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "primer" | "primers" | "primera" | "primeres" => Some(1),
        "segon" | "segona" | "segons" | "segones" => Some(2),
        "tercer" | "tercera" | "tercers" | "terceres" => Some(3),
        "quart" | "quarta" | "quarts" | "quartes" => Some(4),
        "cinquè" | "cinquena" | "cinquens" | "cinquenes" => Some(5),
        "sisè" | "sisena" | "sisens" | "sisenes" => Some(6),
        "setè" | "setena" | "setens" | "setenes" => Some(7),
        "vuitè" | "vuitena" | "vuitens" | "vuitenes" => Some(8),
        "novè" | "novena" | "novens" | "novenes" => Some(9),
        "desè" | "desena" | "desens" | "desenes" => Some(10),
        "onzè" | "onzena" => Some(11),
        "dotzè" | "dotzena" => Some(12),
        "tretzè" | "tretzena" => Some(13),
        "catorzè" | "catorzena" => Some(14),
        "quinzè" | "quinzena" => Some(15),
        "setzè" | "setzena" => Some(16),
        "dissetè" | "dissetena" => Some(17),
        "divuitè" | "divuitena" => Some(18),
        "dinovè" | "dinovena" => Some(19),
        "vintè" | "vintena" => Some(20),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "ordinals (primero..10)".to_string(),
        pattern: vec![regex(
            "(primer(a|s|es)?|segon(a|s|es)?|tercer(a|s|es)?|quart(a|s|es)?|cinqu(è|(en(a|s|es)))|sis(è|en(a|s|es))|set(è|en(a|s|es))|vuit(è|en(a|s|es))|nov(è|en(a|s|es))|des(è|en(a|s|es))|onz(è|ena)|dotz(è|ena)|tretz(è|ena)|catorz(è|ena)|quinz(è|ena)|setz(è|ena)|disset(è|ena)|divuit(è|ena)|dinov(è|ena)|vint(è|ena))",
        )],
        production: Box::new(|nodes| {
            let s = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            Some(TokenData::Ordinal(OrdinalData::new(lookup_ordinal(s)?)))
        }),
    }]
}
