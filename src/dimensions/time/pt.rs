use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use crate::dimensions::time_grain::Grain;
use super::{PartOfDay, TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (pt)".to_string(),
            pattern: vec![regex("agora|j[áa]|nesse instante|neste instante|nesse momento|neste momento")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (pt)".to_string(),
            pattern: vec![regex("hoje")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (pt)".to_string(),
            pattern: vec![regex("amanh[ãa]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (pt)".to_string(),
            pattern: vec![regex("ontem")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (pt)".to_string(),
            pattern: vec![regex("segunda((\\s|\\-)feira)?|seg\\.?|ter(ç|c)a((\\s|\\-)feira)?|ter\\.|quarta((\\s|\\-)feira)?|qua\\.?|quinta((\\s|\\-)feira)?|qui\\.?|sexta((\\s|\\-)feira)?|sex\\.?|s(á|a)bado|s(á|a)b\\.?|domingo|dom\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("seg") || s.starts_with("segunda") {
                    0
                } else if s.starts_with("ter") {
                    1
                } else if s.starts_with("qua") || s.starts_with("quarta") {
                    2
                } else if s.starts_with("qui") || s.starts_with("quinta") {
                    3
                } else if s.starts_with("sex") || s.starts_with("sexta") {
                    4
                } else if s.starts_with("sá") || s.starts_with("sa") {
                    5
                } else if s.starts_with("dom") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "5 de maio (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+de\\s+maio")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day, year: None })))
            }),
        },
        Rule {
            name: "5 maio (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+maio")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day, year: None })))
            }),
        },
        Rule {
            name: "cinco de maio (pt)".to_string(),
            pattern: vec![regex("cinco\\s+de\\s+maio")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day: 5, year: None })))),
        },
        Rule {
            name: "cinco maio (pt)".to_string(),
            pattern: vec![regex("cinco\\s+maio")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day: 5, year: None })))),
        },
        Rule {
            name: "maio 5 (pt)".to_string(),
            pattern: vec![regex("maio\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day, year: None })))
            }),
        },
        Rule {
            name: "maio cinco (pt)".to_string(),
            pattern: vec![regex("maio\\s+cinco")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day: 5, year: None })))),
        },
        Rule {
            name: "4 de julho (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+de\\s+julho")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 7, day, year: None })))
            }),
        },
        Rule {
            name: "04 julho (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+julho")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 7, day, year: None })))
            }),
        },
        Rule {
            name: "julho 4 (pt)".to_string(),
            pattern: vec![regex("julho\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 7, day, year: None })))
            }),
        },
        Rule {
            name: "3 de março (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+de\\s+mar[çc]o")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day, year: None })))
            }),
        },
        Rule {
            name: "três de março (pt)".to_string(),
            pattern: vec![regex("tr[eê]s\\s+de\\s+mar[çc]o")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 3, year: None })))),
        },
        Rule {
            name: "5 de abril (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+de\\s+abril")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day, year: None })))
            }),
        },
        Rule {
            name: "cinco de abril (pt)".to_string(),
            pattern: vec![regex("cinco\\s+de\\s+abril")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day: 5, year: None })))),
        },
        Rule {
            name: "primeiro de março (pt)".to_string(),
            pattern: vec![regex("primeiro\\s+de\\s+mar[çc]o")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "um de março (pt)".to_string(),
            pattern: vec![regex("um\\s+de\\s+mar[çc]o")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "1o de março (pt)".to_string(),
            pattern: vec![regex("1o\\s+de\\s+mar[çc]o")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "essa dia 16 (pt)".to_string(),
            pattern: vec![regex("essa dia\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "este dia 17 (pt)".to_string(),
            pattern: vec![regex("este dia\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "no dia 17 (pt)".to_string(),
            pattern: vec![regex("no dia\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "dia 17 (pt)".to_string(),
            pattern: vec![regex("dia\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "esta semana (pt)".to_string(),
            pattern: vec![regex("esta semana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "semana passada (pt)".to_string(),
            pattern: vec![regex("semana passada")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "semana anterior (pt)".to_string(),
            pattern: vec![regex("semana anterior")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "passada semana (pt)".to_string(),
            pattern: vec![regex("passada semana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "anterior semana (pt)".to_string(),
            pattern: vec![regex("anterior semana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "última semana (pt)".to_string(),
            pattern: vec![regex("última semana|ultima semana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "semana que vem (pt)".to_string(),
            pattern: vec![regex("semana que vem")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "próxima semana (pt)".to_string(),
            pattern: vec![regex("pr[óo]xima semana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "mês passado (pt)".to_string(),
            pattern: vec![regex("m[êe]s passado")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "mês que vem / próximo mês (pt)".to_string(),
            pattern: vec![regex("m[êe]s que vem|pr[óo]ximo m[êe]s")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "ano passado (pt)".to_string(),
            pattern: vec![regex("ano passado")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "este ano (pt)".to_string(),
            pattern: vec![regex("este ano")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "próximo ano (pt)".to_string(),
            pattern: vec![regex("pr[óo]ximo ano|ano que vem")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "às tres da tarde (pt)".to_string(),
            pattern: vec![regex("[àa]s\\s+tr[eê]s\\s+da\\s+tarde")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))),
        },
        Rule {
            name: "às tres (pt)".to_string(),
            pattern: vec![regex("[àa]s\\s+tr[eê]s")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))),
        },
        Rule {
            name: "às tres e quinze (pt)".to_string(),
            pattern: vec![regex("[àa]s\\s+tr[eê]s\\s+e\\s+quinze(\\s+da\\s+tarde)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 15, false))))),
        },
        Rule {
            name: "às tres e meia (pt)".to_string(),
            pattern: vec![regex("[àa]s\\s+tr[eê]s\\s+e\\s+meia(\\s+da\\s+tarde)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 30, false))))),
        },
        Rule {
            name: "às 3 e trinta (pt)".to_string(),
            pattern: vec![regex("[àa]s\\s*3\\s+e\\s+trinta(\\s+da\\s+tarde)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 30, false))))),
        },
        Rule {
            name: "às 15 horas (pt)".to_string(),
            pattern: vec![regex("[àa]s\\s*(\\d{1,2})\\s*horas?")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "às oito da noite (pt)".to_string(),
            pattern: vec![regex("[àa]s\\s+oito\\s+da\\s+noite")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(20, 0, false))))),
        },
        Rule {
            name: "meianoite (pt)".to_string(),
            pattern: vec![regex("meia\\s*noite|meianoite")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(0, 0, false))))),
        },
        Rule {
            name: "meio dia (pt)".to_string(),
            pattern: vec![regex("meio\\s*dia|meiodia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))),
        },
        Rule {
            name: "as seis da manha (pt)".to_string(),
            pattern: vec![regex("a[sà]\\s+seis\\s+(da|pela)\\s+manh[ãa]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(6, 0, false))))),
        },
        Rule {
            name: "6 da manhã (pt)".to_string(),
            pattern: vec![regex("6\\s+da\\s+manh[ãa]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(6, 0, false))))),
        },
        Rule {
            name: "16 de fevereiro (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+de\\s+fevereiro")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day, year: None })))
            }),
        },
        Rule {
            name: "em um minuto (pt)".to_string(),
            pattern: vec![regex("em\\s+um\\s+minuto")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 1,
            })))),
        },
        Rule {
            name: "em 1 min (pt)".to_string(),
            pattern: vec![regex("em\\s*1\\s*min")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 1,
            })))),
        },
        Rule {
            name: "em 2 minutos (pt)".to_string(),
            pattern: vec![regex("em\\s*2\\s*minutos|em\\s+dois\\s+minutos")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 2,
            })))),
        },
        Rule {
            name: "em 60 minutos (pt)".to_string(),
            pattern: vec![regex("em\\s*60\\s*minutos")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 60,
            })))),
        },
        Rule {
            name: "em uma hora (pt)".to_string(),
            pattern: vec![regex("em\\s+uma\\s+hora")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Hour,
                offset: 1,
            })))),
        },
        Rule {
            name: "fazem duas horas (pt)".to_string(),
            pattern: vec![regex("faz(em)?\\s+duas\\s+horas")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Hour,
                offset: -2,
            })))),
        },
        Rule {
            name: "em 24 horas (pt)".to_string(),
            pattern: vec![regex("em\\s*24\\s+horas|em\\s+vinte\\s+e\\s+quatro\\s+horas")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Hour,
                offset: 24,
            })))),
        },
        Rule {
            name: "em um dia (pt)".to_string(),
            pattern: vec![regex("em\\s+um\\s+dia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Day,
                offset: 1,
            })))),
        },
        Rule {
            name: "em 7 dias (pt)".to_string(),
            pattern: vec![regex("em\\s*7\\s+dias")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Day,
                offset: 7,
            })))),
        },
        Rule {
            name: "em uma semana (pt)".to_string(),
            pattern: vec![regex("em\\s+uma\\s+semana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Week,
                offset: 1,
            })))),
        },
        Rule {
            name: "faz tres semanas (pt)".to_string(),
            pattern: vec![regex("faz\\s+tr[eê]s\\s+semanas")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Week,
                offset: -3,
            })))),
        },
        Rule {
            name: "em dois meses (pt)".to_string(),
            pattern: vec![regex("em\\s+dois\\s+meses")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Month,
                offset: 2,
            })))),
        },
        Rule {
            name: "faz tres meses (pt)".to_string(),
            pattern: vec![regex("faz\\s+tr[eê]s\\s+meses")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Month,
                offset: -3,
            })))),
        },
        Rule {
            name: "em um ano (pt)".to_string(),
            pattern: vec![regex("em\\s+(um|1)\\s+ano")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Year,
                offset: 1,
            })))),
        },
        Rule {
            name: "faz dois anos (pt)".to_string(),
            pattern: vec![regex("faz\\s+dois\\s+anos")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Year,
                offset: -2,
            })))),
        },
        Rule {
            name: "este verão (pt)".to_string(),
            pattern: vec![regex("este\\s+ver[ãa]o|ver[ãa]o")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(1))))),
        },
        Rule {
            name: "este inverno (pt)".to_string(),
            pattern: vec![regex("este\\s+inverno|inverno")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(3))))),
        },
        Rule {
            name: "natal (pt)".to_string(),
            pattern: vec![regex("natal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "christmas day".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "véspera de ano novo (pt)".to_string(),
            pattern: vec![regex("v[ée]spera\\s+de\\s+ano\\s+novo")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "new year's eve".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "ano novo / reveillon (pt)".to_string(),
            pattern: vec![regex("ano\\s+novo|reveillon")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "new year".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "esta noite (pt)".to_string(),
            pattern: vec![regex("(esta|essa)\\s+noite")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))))),
        },
        Rule {
            name: "amanhã à noite (pt)".to_string(),
            pattern: vec![regex("amanh[ãa]\\s+[àa]\\s+noite")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Tomorrow)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "ontem à noite (pt)".to_string(),
            pattern: vec![regex("ontem\\s+[àa]\\s+noite")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Yesterday)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "amanhã à tarde (pt)".to_string(),
            pattern: vec![regex("amanh[ãa]\\s+[àa]\\s+tarde")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Tomorrow)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))),
            ))))),
        },
        Rule {
            name: "ontem à tarde (pt)".to_string(),
            pattern: vec![regex("ontem\\s+[àa]\\s+tarde")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Yesterday)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))),
            ))))),
        },
        Rule {
            name: "este fim de semana (pt)".to_string(),
            pattern: vec![regex("este\\s+(final\\s+de\\s+semana|fim\\s+de\\s+semana)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "segunda de manhã (pt)".to_string(),
            pattern: vec![regex("segunda\\s+de\\s+manh[ãa]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::DayOfWeek(0))),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))),
            ))))),
        },
        Rule {
            name: "dia 15 de fevereiro de manhã (pt)".to_string(),
            pattern: vec![regex("dia\\s+15\\s+de\\s+fevereiro\\s+(pela\\s+|de\\s+)?manh[ãa]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day: 15,
                    year: None,
                })),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))),
            ))))),
        },
        Rule {
            name: "às 8 da noite (pt)".to_string(),
            pattern: vec![regex("[àa]s\\s*8\\s+da\\s+noite")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(20, 0, false))))),
        },
        Rule {
            name: "2 segundos atrás (pt)".to_string(),
            pattern: vec![regex("2\\s+segundos?\\s+atr[aá]s")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Second, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "proximos 3 segundos (pt)".to_string(),
            pattern: vec![regex("pr[óo]xim[oa]s?\\s+3\\s+segundos?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Second, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "2 minutos atrás (pt)".to_string(),
            pattern: vec![regex("2\\s+minutos?\\s+atr[aá]s")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "proximos 3 minutos (pt)".to_string(),
            pattern: vec![regex("pr[óo]xim[oa]s?\\s+3\\s+minutos?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "proximas 3 horas (pt)".to_string(),
            pattern: vec![regex("pr[óo]xim[oa]s?\\s+3\\s+horas?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "passados 2 dias (pt)".to_string(),
            pattern: vec![regex("passad[oa]s?\\s+2\\s+dias?|2\\s+dias?\\s+anteriores")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "proximos 3 dias (pt)".to_string(),
            pattern: vec![regex("pr[óo]xim[oa]s?\\s+3\\s+dias?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "duas semanas atras (pt)".to_string(),
            pattern: vec![regex("duas\\s+semanas\\s+atr[aá]s|2\\s+semanas\\s+anteriores|[úu]ltim[oa]s?\\s+2\\s+semanas|2\\s+[úu]ltim[oa]s?\\s+semanas|2\\s+anteriores\\s+semanas")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "3 proximas semanas (pt)".to_string(),
            pattern: vec![regex("3\\s+pr[óo]xim[oa]s?\\s+semanas|3\\s+semanas\\s+que\\s+vem")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "2 meses anteriores (pt)".to_string(),
            pattern: vec![regex("passad[oa]s?\\s+2\\s+meses|[úu]ltim[oa]s?\\s+2\\s+meses|2\\s+meses\\s+anteriores|2\\s+[úu]ltim[oa]s?\\s+meses|2\\s+anteriores\\s+meses")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "3 proximos meses (pt)".to_string(),
            pattern: vec![regex("3\\s+pr[óo]xim[oa]s?\\s+meses|pr[óo]ximos\\s+tr[eê]s\\s+meses|tr[eê]s\\s+meses\\s+que\\s+vem")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "2 anos anteriores (pt)".to_string(),
            pattern: vec![regex("passad[oa]s?\\s+2\\s+anos|[úu]ltim[oa]s?\\s+2\\s+anos|2\\s+anos\\s+anteriores|2\\s+[úu]ltim[oa]s?\\s+anos|2\\s+anteriores\\s+anos")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "3 proximos anos (pt)".to_string(),
            pattern: vec![regex("3\\s+pr[óo]xim[oa]s?\\s+anos|pr[óo]ximo\\s+tr[eê]s\\s+anos|3\\s+anos\\s+que\\s+vem")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "dentro de tres horas (pt)".to_string(),
            pattern: vec![regex("dentro\\s+de\\s+tr[eê]s\\s+horas")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "última hora (pt)".to_string(),
            pattern: vec![regex("[úu]ltima\\s+hora|hora\\s+anterior|hora\\s+passada")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Hour,
                offset: -1,
            })))),
        },
        Rule {
            name: "este trimestre (pt)".to_string(),
            pattern: vec![regex("este\\s+trimestre|trimestre\\s+act?ual|trimestre\\s+atual")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Quarter,
                offset: 0,
            })))),
        },
        Rule {
            name: "primeiro mês de 2013 (pt)".to_string(),
            pattern: vec![regex("primeiro\\s+m[êe]s(\\s+de)?\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(1))),
                ))))
            }),
        },
        Rule {
            name: "próximo trimestre (pt)".to_string(),
            pattern: vec![regex("pr[óo]ximo\\s+trimestre")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Quarter,
                offset: 1,
            })))),
        },
        Rule {
            name: "segundo trimestre (pt)".to_string(),
            pattern: vec![regex("segundo\\s+trimestre(\\s+de\\s+(\\d{4}))?")],
            production: Box::new(|nodes| {
                let year_opt = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2),
                    _ => return None,
                };
                if let Some(y) = year_opt {
                    let year: i32 = y.parse().ok()?;
                    Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(2, year))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::Quarter(2))))
                }
            }),
        },
        Rule {
            name: "terceiro trimestre (pt)".to_string(),
            pattern: vec![regex("terceiro\\s+trimestre")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "quarto trimestre 2018 (pt)".to_string(),
            pattern: vec![regex("quarto\\s+trimestre(\\s+de)?\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(4, year))))
            }),
        },
        Rule {
            name: "trimestre passado (pt)".to_string(),
            pattern: vec![regex("trimestre\\s+passado|trimestre\\s+anterior|[úu]ltimo\\s+trimestre")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Quarter,
                offset: -1,
            })))),
        },
        Rule {
            name: "último mês de 2013 (pt)".to_string(),
            pattern: vec![regex("d[eé]cimo\\s+segundo\\s+m[êe]s\\s+de\\s+(\\d{4})|[úu]ltimo\\s+m[êe]s(\\s+de)?\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let year_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => {
                        if let Some(y1) = m.group(1) {
                            y1
                        } else {
                            m.group(3)?
                        }
                    }
                    _ => return None,
                };
                let year: i32 = year_s.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(12))),
                ))))
            }),
        },
        Rule {
            name: "último trimestre de 2015 (pt)".to_string(),
            pattern: vec![regex("[úu]ltimo\\s+trimestre(\\s+de)?\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(4, year))))
            }),
        },
        Rule {
            name: "terceiro mês de 2017 até nono mês de 2017 (pt)".to_string(),
            pattern: vec![regex("((de|do|desde|a\\s+partir\\s+d[eo]|entre(\\s+o)?)\\s+)?terceiro\\s+m[êe]s(\\s+de)?\\s+2017\\s*(a|ao|at[eé](\\s+ao)?|\\-|e)\\s+nono\\s+m[êe]s(\\s+de)?\\s+2017")],
            production: Box::new(|_| {
                let from = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(2017))),
                    Box::new(TimeData::new(TimeForm::Month(3))),
                ));
                let to = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(2017))),
                    Box::new(TimeData::new(TimeForm::Month(9))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
    ]);
    rules
}
