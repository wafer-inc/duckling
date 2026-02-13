use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "ordinals (primero..10)".to_string(),
        pattern: vec![regex(
            "((primer|segund|tercer|cuart|quint|sext|s[eé]ptim|octav|noven|d[eé]cim)(os?|as?)|(prim|terc)er)",
        )],
        production: Box::new(|nodes| {
            let text = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            let value = match text.to_lowercase().as_str() {
                "primer" | "primero" | "primeros" | "primera" | "primeras" => 1,
                "segundo" | "segunda" | "segundas" | "segundos" => 2,
                "terceros" | "tercera" | "terceras" | "tercero" | "tercer" => 3,
                "cuarta" | "cuartas" | "cuartos" | "cuarto" => 4,
                "quinto" | "quinta" | "quintas" | "quintos" => 5,
                "sextos" | "sexto" | "sexta" | "sextas" => 6,
                "séptimas" | "septimas" | "séptima" | "septimos" | "septima" | "séptimo"
                | "séptimos" | "septimo" => 7,
                "octavas" | "octavo" | "octavos" | "octava" => 8,
                "novenos" | "novena" | "noveno" | "novenas" => 9,
                "décimos" | "decimo" | "decimos" | "décimo" | "decimas" | "décima"
                | "decima" | "décimas" => 10,
                _ => return None,
            };
            Some(TokenData::Ordinal(OrdinalData::new(value)))
        }),
    }]
}
