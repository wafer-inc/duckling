use super::{Direction, IntervalDirection, PartOfDay, TimeData, TimeForm};
use crate::dimensions::numeral::helpers::numeral_data;
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (fr)".to_string(),
            pattern: vec![regex("maintenant|tout de suite|dans la journée|en ce moment")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (fr)".to_string(),
            pattern: vec![regex("aujourd'hui|ce jour")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (fr)".to_string(),
            pattern: vec![regex("(le len)?demain|jour suivant|le jour d'apr(e|é|è)s|un jour apr(e|é|è)s")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (fr)".to_string(),
            pattern: vec![regex("hier|le jour d'avant|le jour précédent|la veille")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week monday (fr)".to_string(),
            pattern: vec![regex("lundi|lun\\.")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(0))))),
        },
        Rule {
            name: "day of week tuesday (fr)".to_string(),
            pattern: vec![regex("mardi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(1))))),
        },
        Rule {
            name: "day of week wednesday (fr)".to_string(),
            pattern: vec![regex("mercredi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(2))))),
        },
        Rule {
            name: "day of week thursday (fr)".to_string(),
            pattern: vec![regex("jeudi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(3))))),
        },
        Rule {
            name: "day of week friday (fr)".to_string(),
            pattern: vec![regex("vendredi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(4))))),
        },
        Rule {
            name: "day of week saturday (fr)".to_string(),
            pattern: vec![regex("samedi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(5))))),
        },
        Rule {
            name: "day of week sunday (fr)".to_string(),
            pattern: vec![regex("dimanche")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(6))))),
        },
        Rule {
            name: "<day-of-week> prochain (fr)".to_string(),
            pattern: vec![regex("(lundi|mardi|mercredi|jeudi|vendredi|samedi|dimanche)\\s+prochain")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let dow = match s {
                    "lundi" => 0,
                    "mardi" => 1,
                    "mercredi" => 2,
                    "jeudi" => 3,
                    "vendredi" => 4,
                    "samedi" => 5,
                    "dimanche" => 6,
                    _ => return None,
                };
                let mut td = TimeData::new(TimeForm::DayOfWeek(dow));
                td.direction = Some(Direction::Future);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "<day-of-week> dernier (fr)".to_string(),
            pattern: vec![regex("(lundi|mardi|mercredi|jeudi|vendredi|samedi|dimanche)\\s+dernier")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let dow = match s {
                    "lundi" => 0,
                    "mardi" => 1,
                    "mercredi" => 2,
                    "jeudi" => 3,
                    "vendredi" => 4,
                    "samedi" => 5,
                    "dimanche" => 6,
                    _ => return None,
                };
                let mut td = TimeData::new(TimeForm::DayOfWeek(dow));
                td.direction = Some(Direction::Past);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "le 1er mars (fr)".to_string(),
            pattern: vec![regex("le\\s+1er\\s+mars")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "premier mars (fr)".to_string(),
            pattern: vec![regex("premier mars")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "le 1 mars (fr)".to_string(),
            pattern: vec![regex("le\\s+1\\s+mars")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "le 2 mars (fr)".to_string(),
            pattern: vec![regex("le\\s+2\\s+mars")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 2, year: None })))),
        },
        Rule {
            name: "2 mars (fr)".to_string(),
            pattern: vec![regex("2\\s+mars")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 2, year: None })))),
        },
        Rule {
            name: "3 mars (fr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+mars")],
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
            name: "5 avril (fr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+avril")],
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
            name: "15 février (fr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+f[ée]vrier")],
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
            name: "15 fev 2013 (fr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+f[ée]v\\.?\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (d, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let year: i32 = y.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "17 02 (fr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (d, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "mercredi 13 (fr)".to_string(),
            pattern: vec![regex("mercredi\\s+(\\d{1,2})")],
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
            name: "31 octobre (fr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+octobre")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day, year: None })))
            }),
        },
        Rule {
            name: "le 5 juillet (fr)".to_string(),
            pattern: vec![regex("le\\s+5\\s+juillet")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 7, day: 5, year: None })))),
        },
        Rule {
            name: "5 juillet (fr)".to_string(),
            pattern: vec![regex("5\\s+juillet")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 7, day: 5, year: None })))),
        },
        Rule {
            name: "5 juil (fr)".to_string(),
            pattern: vec![regex("5\\s+juil\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 7, day: 5, year: None })))),
        },
        Rule {
            name: "5 jui (fr)".to_string(),
            pattern: vec![regex("5\\s+jui\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 7, day: 5, year: None })))),
        },
        Rule {
            name: "le 2 (fr)".to_string(),
            pattern: vec![regex("le\\s*(\\d{1,2})")],
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
            name: "mercredi 13 février (fr)".to_string(),
            pattern: vec![regex("mercredi\\s+(\\d{1,2})\\s+f[ée]vrier")],
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
            name: "deux jours plus tard (fr)".to_string(),
            pattern: vec![regex("deux jours plus tard")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: 2 })))),
        },
        Rule {
            name: "deux jours après (fr)".to_string(),
            pattern: vec![regex("deux jours après|deux jours apres")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: 2 })))),
        },
        Rule {
            name: "cette semaine (fr)".to_string(),
            pattern: vec![regex("cette semaine|dans la semaine")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "la semaine prochaine (fr)".to_string(),
            pattern: vec![regex("(la )?semaine (prochaine|suivante|qui suit)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "la semaine dernière (fr)".to_string(),
            pattern: vec![regex("(la )?semaine (derni(è|e)re|pr(é|e)c(é|e)dente)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "ce mois (fr)".to_string(),
            pattern: vec![regex("(ce|ceci) mois")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Month))))),
        },
        Rule {
            name: "le mois prochain (fr)".to_string(),
            pattern: vec![regex("(le )?mois (prochain|suivant)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "le mois dernier (fr)".to_string(),
            pattern: vec![regex("(le )?mois (dernier|pr(é|e)c(é|e)dent)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "cette année (fr)".to_string(),
            pattern: vec![regex("cette ann(é|e)e")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "l'année prochaine (fr)".to_string(),
            pattern: vec![regex("l'?ann(é|e)e (prochaine|suivante)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "l'année dernière (fr)".to_string(),
            pattern: vec![regex("l'?ann(é|e)e (derni(è|e)re|pr(é|e)c(é|e)dente)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "named month (fr)".to_string(),
            pattern: vec![regex("janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "plus tard (fr)".to_string(),
            pattern: vec![regex("plus tard")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: 2, grain: Grain::Hour })))),
        },
        Rule {
            name: "plus tard dans l'après-midi/soirée (fr)".to_string(),
            pattern: vec![regex("(un peu )?plus tard dans l[' ]apr(è|e)s-?midi|(un peu )?plus tard dans la soir(é|e)e")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let pod = if s.contains("soir") {
                    PartOfDay::Evening
                } else {
                    PartOfDay::Afternoon
                };
                Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(pod))))
            }),
        },
        Rule {
            name: "week span variants (fr)".to_string(),
            pattern: vec![regex("(en|au) d(é|e)but de (la )?semaine|(en|au) milieu de (la )?semaine|(en|(à|a) la) fin de (la )?semaine|en semaine")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "weekend variants (fr)".to_string(),
            pattern: vec![regex("ce week-?end|le premier week-?end de|le deuxi(è|e)me week-?end de|le dernier week-?end de")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "<ordinal> jour d'<month> (fr)".to_string(),
            pattern: vec![regex("(le )?(\\d{1,2})(er|e|eme|ème)\\s+jour\\s+d[e']\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)")],
            production: Box::new(|nodes| {
                let (d, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(4)?.to_lowercase()),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "dans <duration> (fr)".to_string(),
            pattern: vec![regex("dans"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let d = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: d.value,
                    grain: d.grain,
                })))
            }),
        },
        Rule {
            name: "dans les 15 jours (fr)".to_string(),
            pattern: vec![regex("dans les\\s*(\\d{1,4})\\s+jours")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let days: i64 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: days,
                    grain: Grain::Day,
                })))
            }),
        },
        Rule {
            name: "il y a <duration> (fr)".to_string(),
            pattern: vec![regex("il y a"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let d = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: d.value.checked_neg()?,
                    grain: d.grain,
                })))
            }),
        },
        Rule {
            name: "d'ici <duration> (fr)".to_string(),
            pattern: vec![regex("d'ici"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let d = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::Now)),
                    Box::new(TimeData::new(TimeForm::RelativeGrain {
                        n: d.value,
                        grain: d.grain,
                    })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "<integer> prochaines <cycle> (fr)".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("prochain(e|es|s)?"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value;
                if n < 1.0 || n.fract() != 0.0 {
                    return None;
                }
                let g = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: n as i64,
                    grain: g,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "<integer> dernières <cycle> (fr)".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("derni(è|e)r(e|es|s)?|pass(é|e)es?"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value;
                if n < 1.0 || n.fract() != 0.0 {
                    return None;
                }
                let g = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: n as i64,
                    grain: g,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "première semaine de <month> <year> (fr)".to_string(),
            pattern: vec![regex("(la )?premi(è|e)re\\s+semaine\\s+d[e']\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let (m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(3)?.to_lowercase(), rm.group(4)?),
                    _ => return None,
                };
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime {
                    n: 1,
                    grain: Grain::Week,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "la semaine du <day> <month> (fr)".to_string(),
            pattern: vec![regex("la\\s+semaine\\s+du\\s*(\\d{1,2})\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)")],
            production: Box::new(|nodes| {
                let (d, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "dernier jour de <month> <year> (fr)".to_string(),
            pattern: vec![regex("(le )?dernier\\s+jour\\s+d[e']\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let (m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?.to_lowercase(), rm.group(3)?),
                    _ => return None,
                };
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::NthLastDayOfTime {
                    n: 1,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "dernière semaine de <month> <year> (fr)".to_string(),
            pattern: vec![regex("(la )?derni(è|e)re\\s+semaine\\s+de\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let (m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(3)?.to_lowercase(), rm.group(4)?),
                    _ => return None,
                };
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::NthLastCycleOfTime {
                    n: 1,
                    grain: Grain::Week,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "à <integer> heures (fr)".to_string(),
            pattern: vec![regex("(à|a)"), dim(DimensionKind::Numeral), regex("heures?")],
            production: Box::new(|nodes| {
                let h = numeral_data(&nodes[1].token_data)?.value;
                if !(0.0..=23.0).contains(&h) || h.fract() != 0.0 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                    h as u32, 0, false,
                ))))
            }),
        },
        Rule {
            name: "<integer> heures (fr)".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("heures?")],
            production: Box::new(|nodes| {
                let h = numeral_data(&nodes[0].token_data)?.value;
                if !(0.0..=23.0).contains(&h) || h.fract() != 0.0 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                    h as u32, 0, false,
                ))))
            }),
        },
        Rule {
            name: "midi (fr)".to_string(),
            pattern: vec![regex("midi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))),
        },
        Rule {
            name: "minuit (fr)".to_string(),
            pattern: vec![regex("minuit")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(0, 0, false))))),
        },
        Rule {
            name: "season (fr)".to_string(),
            pattern: vec![regex("(cet|ce|cette|prochain|dernier)?\\s*(printemps|[ée]t[ée]|automne|hiver)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?.to_lowercase(),
                    _ => return None,
                };
                let season = match s.as_str() {
                    "printemps" => 0,
                    "été" | "ete" => 1,
                    "automne" => 2,
                    "hiver" => 3,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "christmas/newyear holidays (fr)".to_string(),
            pattern: vec![regex("no[ëe]l|nouvel an|jour de l'an|saint\\s+sylvestre")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let name = if s.contains("no") {
                    "christmas day"
                } else if s.contains("sylvestre") {
                    "new year's eve"
                } else {
                    "new year's day"
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    name.to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "toussaint/workday holidays (fr)".to_string(),
            pattern: vec![regex("toussaint|jour des morts|f(ê|e)te du travail")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let name = if s.contains("toussaint") || s.contains("morts") {
                    "all saints day"
                } else {
                    "labor day"
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    name.to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "morning variants (fr)".to_string(),
            pattern: vec![regex("en d(é|e)but de matin(é|e)e|en milieu de matin(é|e)e|en fin de matin(é|e)e|dans la matin(é|e)e|le matin|d(è|e)s la matin(é|e)e")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))))),
        },
        Rule {
            name: "afternoon variants (fr)".to_string(),
            pattern: vec![regex("cet? apr(è|e)s-?midi|en d(é|e)but d[' ]apr(è|e)s-?midi|en milieu d[' ]apr(è|e)s-?midi|en fin d[' ]apr(è|e)s-?midi|dans l[' ]apr(è|e)s-?midi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))))),
        },
        Rule {
            name: "evening variants (fr)".to_string(),
            pattern: vec![regex("ce soir|le soir|demain soir|hier soir|(en|au) d(é|e)but de (la )?soir(é|e)e|(en|(à|a) la) fin de (la )?soir(é|e)e|dans la soir(é|e)e")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))))),
        },
        Rule {
            name: "day-part variants (fr)".to_string(),
            pattern: vec![regex("(en|au) d(é|e)but de (la )?journ(é|e)e|(en|au) milieu de (la )?journ(é|e)e|(en|(à|a) la) fin de (la )?journ(é|e)e")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))))),
        },
        Rule {
            name: "lunch variants (fr)".to_string(),
            pattern: vec![regex("apr(è|e)s d(é|e)jeuner|avant d(é|e)jeuner|pendant le d(é|e)jeuner|(à|a) l'heure du d(é|e)jeuner|pendant d(é|e)jeuner|au moment de d(é|e)jeuner")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Lunch))))),
        },
        Rule {
            name: "after work (fr)".to_string(),
            pattern: vec![regex("apr(è|e)s le travail")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))))),
        },
        Rule {
            name: "<ordinal> <month> (fr)".to_string(),
            pattern: vec![
                dim(DimensionKind::Ordinal),
                regex("(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)"),
            ],
            production: Box::new(|nodes| {
                let day = match &nodes[0].token_data {
                    TokenData::Ordinal(o) => o.value as u32,
                    _ => return None,
                };
                if !(1..=31).contains(&day) {
                    return None;
                }
                let m = match &nodes[1].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "<n> dernières/prochaines <cycle> literal (fr)".to_string(),
            pattern: vec![regex("(\\d{1,4})\\s+(derni(è|e)res?|prochaines?)\\s+(secondes?|minutes?|heures?|jours?|semaines?|mois|ann(é|e)es?)")],
            production: Box::new(|nodes| {
                let (n, dir, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?.to_lowercase(), m.group(4)?.to_lowercase()),
                    _ => return None,
                };
                let count: i64 = n.parse().ok()?;
                let grain = if unit.starts_with("seconde") {
                    Grain::Second
                } else if unit.starts_with("minute") {
                    Grain::Minute
                } else if unit.starts_with("heure") {
                    Grain::Hour
                } else if unit.starts_with("jour") {
                    Grain::Day
                } else if unit.starts_with("semaine") {
                    Grain::Week
                } else if unit == "mois" {
                    Grain::Month
                } else if unit.starts_with("ann") {
                    Grain::Year
                } else {
                    return None;
                };
                let past = dir.starts_with("derni");
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: count,
                    grain,
                    past,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "<word-num> dernières/prochaines <cycle> (fr)".to_string(),
            pattern: vec![regex("(un|une|deux|trois)\\s+(derni(è|e)res?|prochaines?)\\s+(secondes?|minutes?|heures?|jours?|semaines?|mois|ann(é|e)es?)")],
            production: Box::new(|nodes| {
                let (n, dir, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.to_lowercase(), m.group(2)?.to_lowercase(), m.group(4)?.to_lowercase()),
                    _ => return None,
                };
                let count = match n.as_str() {
                    "un" | "une" => 1,
                    "deux" => 2,
                    "trois" => 3,
                    _ => return None,
                };
                let grain = if unit.starts_with("seconde") {
                    Grain::Second
                } else if unit.starts_with("minute") {
                    Grain::Minute
                } else if unit.starts_with("heure") {
                    Grain::Hour
                } else if unit.starts_with("jour") {
                    Grain::Day
                } else if unit.starts_with("semaine") {
                    Grain::Week
                } else if unit == "mois" {
                    Grain::Month
                } else if unit.starts_with("ann") {
                    Grain::Year
                } else {
                    return None;
                };
                let past = dir.starts_with("derni");
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: count,
                    grain,
                    past,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "<n> <cycle> passées/suivantes (fr)".to_string(),
            pattern: vec![regex("(\\d{1,4})\\s+(secondes?|minutes?|heures?|jours?|semaines?|mois|ann(é|e)es?)\\s+(pass(é|e)es?|suivant(e|es|s)?)")],
            production: Box::new(|nodes| {
                let (n, unit, dir) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?.to_lowercase(), m.group(4)?.to_lowercase()),
                    _ => return None,
                };
                let count: i64 = n.parse().ok()?;
                let grain = if unit.starts_with("seconde") {
                    Grain::Second
                } else if unit.starts_with("minute") {
                    Grain::Minute
                } else if unit.starts_with("heure") {
                    Grain::Hour
                } else if unit.starts_with("jour") {
                    Grain::Day
                } else if unit.starts_with("semaine") {
                    Grain::Week
                } else if unit == "mois" {
                    Grain::Month
                } else if unit.starts_with("ann") {
                    Grain::Year
                } else {
                    return None;
                };
                let past = dir.starts_with("pass");
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: count,
                    grain,
                    past,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "13-15 juillet (fr)".to_string(),
            pattern: vec![regex("(\\d{1,2})(er)?\\s*[-–]\\s*(\\d{1,2})(er)?\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)")],
            production: Box::new(|nodes| {
                let (d1, d2, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?, rm.group(5)?.to_lowercase()),
                    _ => return None,
                };
                let from_day: u32 = d1.parse().ok()?;
                let to_day: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&from_day) || !(1..=31).contains(&to_day) {
                    return None;
                }
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::DateMDY { month, day: from_day, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: to_day, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "1er au 10 juillet (fr)".to_string(),
            pattern: vec![regex("(\\d{1,2})(er)?\\s+au\\s+(\\d{1,2})(er)?\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)")],
            production: Box::new(|nodes| {
                let (d1, d2, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?, rm.group(5)?.to_lowercase()),
                    _ => return None,
                };
                let from_day: u32 = d1.parse().ok()?;
                let to_day: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&from_day) || !(1..=31).contains(&to_day) {
                    return None;
                }
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::DateMDY { month, day: from_day, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: to_day, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "du 13 au 18 (fr)".to_string(),
            pattern: vec![regex("du\\s+(\\d{1,2})(er)?\\s+au\\s+(\\d{1,2})(er)?")],
            production: Box::new(|nodes| {
                let (d1, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?),
                    _ => return None,
                };
                let from_day: u32 = d1.parse().ok()?;
                let to_day: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&from_day) || !(1..=31).contains(&to_day) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DayOfMonth(from_day));
                let to = TimeData::new(TimeForm::DayOfMonth(to_day));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "10 juin au 1er juillet (fr)".to_string(),
            pattern: vec![regex("(\\d{1,2})(er)?\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)\\s+au\\s+(\\d{1,2})(er)?\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)")],
            production: Box::new(|nodes| {
                let (d1, m1, d2, m2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?.to_lowercase(), rm.group(4)?, rm.group(6)?.to_lowercase()),
                    _ => return None,
                };
                let from_day: u32 = d1.parse().ok()?;
                let to_day: u32 = d2.parse().ok()?;
                let month_num = |m: &str| -> Option<u32> {
                    Some(match m {
                        "janvier" => 1,
                        "février" | "fevrier" => 2,
                        "mars" => 3,
                        "avril" => 4,
                        "mai" => 5,
                        "juin" => 6,
                        "juillet" => 7,
                        "août" | "aout" => 8,
                        "septembre" => 9,
                        "octobre" => 10,
                        "novembre" => 11,
                        "décembre" | "decembre" => 12,
                        _ => return None,
                    })
                };
                let from_month = month_num(&m1)?;
                let to_month = month_num(&m2)?;
                let from = TimeData::new(TimeForm::DateMDY { month: from_month, day: from_day, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month: to_month, day: to_day, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "du 1er au dix juillet (fr)".to_string(),
            pattern: vec![regex("(du\\s+)?(\\d{1,2})(er)?\\s+au\\s+(un|deux|trois|quatre|cinq|six|sept|huit|neuf|dix)\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)")],
            production: Box::new(|nodes| {
                let (d1, d2w, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(4)?, rm.group(5)?.to_lowercase()),
                    _ => return None,
                };
                let from_day: u32 = d1.parse().ok()?;
                let to_day: u32 = match d2w {
                    "un" => 1,
                    "deux" => 2,
                    "trois" => 3,
                    "quatre" => 4,
                    "cinq" => 5,
                    "six" => 6,
                    "sept" => 7,
                    "huit" => 8,
                    "neuf" => 9,
                    "dix" => 10,
                    _ => return None,
                };
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::DateMDY { month, day: from_day, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: to_day, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "du 13e au dix-huit (fr)".to_string(),
            pattern: vec![regex("du\\s+(\\d{1,2})(er|e|eme|ème)?\\s+au\\s+(dix-huit|dix sept|dix-sept|seize|quinze|quatorze|treize|douze|onze|dix|neuf|huit|sept|six|cinq|quatre|trois|deux|un)")],
            production: Box::new(|nodes| {
                let (d1, d2w) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let from_day: u32 = d1.parse().ok()?;
                let to_day: u32 = match d2w.as_str() {
                    "un" => 1,
                    "deux" => 2,
                    "trois" => 3,
                    "quatre" => 4,
                    "cinq" => 5,
                    "six" => 6,
                    "sept" => 7,
                    "huit" => 8,
                    "neuf" => 9,
                    "dix" => 10,
                    "onze" => 11,
                    "douze" => 12,
                    "treize" => 13,
                    "quatorze" => 14,
                    "quinze" => 15,
                    "seize" => 16,
                    "dix-sept" | "dix sept" => 17,
                    "dix-huit" => 18,
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::DayOfMonth(from_day));
                let to = TimeData::new(TimeForm::DayOfMonth(to_day));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "14 - 20 sept. 2014 (fr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*[-–]\\s*(\\d{1,2})\\s*(sept\\.?|janv\\.?|f[ée]vr\\.?|mars|avr\\.?|mai|juin|juil\\.?|ao[uû]t|oct\\.?|nov\\.?|d[ée]c\\.?|septembre|janvier|f[ée]vrier|avril|juillet|octobre|novembre|d[ée]cembre)\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let (d1, d2, m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?.to_lowercase(), rm.group(4)?),
                    _ => return None,
                };
                let from_day: u32 = d1.parse().ok()?;
                let to_day: u32 = d2.parse().ok()?;
                let year: i32 = y.parse().ok()?;
                let month = match m.as_str() {
                    "janv" | "janv." | "janvier" => 1,
                    "févr" | "fevr" | "févr." | "fevr." | "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avr" | "avr." | "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juil" | "juil." | "juillet" => 7,
                    "août" | "aout" => 8,
                    "sept" | "sept." | "septembre" => 9,
                    "oct" | "oct." | "octobre" => 10,
                    "nov" | "nov." | "novembre" => 11,
                    "déc" | "dec" | "déc." | "dec." | "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::DateMDY { month, day: from_day, year: Some(year) });
                let to = TimeData::new(TimeForm::DateMDY { month, day: to_day, year: Some(year) });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "de 5 à 7 (fr)".to_string(),
            pattern: vec![regex("de\\s*(\\d{1,2})\\s*(h)?\\s*(\\d{0,2})\\s*(à|a)\\s*(\\d{1,2})\\s*(h)?\\s*(\\d{0,2})")],
            production: Box::new(|nodes| {
                let (h1, m1, h2, m2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (
                        rm.group(1)?,
                        rm.group(3).unwrap_or(""),
                        rm.group(5)?,
                        rm.group(7).unwrap_or(""),
                    ),
                    _ => return None,
                };
                let from_h: u32 = h1.parse().ok()?;
                let to_h: u32 = h2.parse().ok()?;
                let from_m: u32 = if m1.is_empty() { 0 } else { m1.parse().ok()? };
                let to_m: u32 = if m2.is_empty() { 0 } else { m2.parse().ok()? };
                let from = TimeData::new(TimeForm::HourMinute(from_h, from_m, false));
                let to = TimeData::new(TimeForm::HourMinute(to_h, to_m, false));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "entre midi et 2 (fr)".to_string(),
            pattern: vec![regex("entre\\s+midi\\s+et\\s+(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let to_h: u32 = h.parse().ok()?;
                let from = TimeData::new(TimeForm::HourMinute(12, 0, false));
                let to = TimeData::new(TimeForm::HourMinute(to_h, 0, false));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "11h30-1h30 (fr)".to_string(),
            pattern: vec![regex("(\\d{1,2})h(\\d{2})\\s*[-–]\\s*(\\d{1,2})h(\\d{2})")],
            production: Box::new(|nodes| {
                let (h1, m1, h2, m2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?, m.group(3)?, m.group(4)?),
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::HourMinute(h1.parse().ok()?, m1.parse().ok()?, false));
                let to = TimeData::new(TimeForm::HourMinute(h2.parse().ok()?, m2.parse().ok()?, false));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "fin/début/mi <month> (fr)".to_string(),
            pattern: vec![regex("(fin|d(é|e)but|mi[- ]?)\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)")],
            production: Box::new(|nodes| {
                let (pos, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let mut td = TimeData::new(TimeForm::Month(month));
                td.early_late = Some(if pos.starts_with("fin") {
                    super::EarlyLate::Late
                } else if pos.starts_with("d") {
                    super::EarlyLate::Early
                } else {
                    super::EarlyLate::Mid
                });
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "en fin/début de mois/année (fr)".to_string(),
            pattern: vec![regex("(en|(à|a) la|au)\\s*(fin|d(é|e)but)\\s+(du|de l'|de la|d[e'])\\s*(mois|ann(é|e)e)")],
            production: Box::new(|nodes| {
                let (begin, gtxt) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(3)?.to_lowercase().starts_with("d"), rm.group(5)?.to_lowercase()),
                    _ => return None,
                };
                let grain = if gtxt.starts_with("mois") { Grain::Month } else { Grain::Year };
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin,
                    target: Box::new(TimeForm::GrainOffset { grain, offset: 0 }),
                })))
            }),
        },
        Rule {
            name: "fin/début du mois de <month> (fr)".to_string(),
            pattern: vec![regex("(fin|d(é|e)but)\\s+du\\s+mois\\s+d[e']\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)")],
            production: Box::new(|nodes| {
                let (pos, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let mut td = TimeData::new(TimeForm::Month(month));
                td.early_late = Some(if pos.starts_with("fin") {
                    super::EarlyLate::Late
                } else {
                    super::EarlyLate::Early
                });
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "première/deuxième quinzaine d'<month> (fr)".to_string(),
            pattern: vec![regex("la\\s+(premi(è|e)re|deuxi(è|e)me)\\s+quinzaine\\s+d[e']\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)")],
            production: Box::new(|nodes| {
                let (half, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(5)?.to_lowercase()),
                    _ => return None,
                };
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let mut td = TimeData::new(TimeForm::Month(month));
                td.early_late = Some(if half.starts_with("premi") {
                    super::EarlyLate::Early
                } else {
                    super::EarlyLate::Late
                });
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "première quinzaine d'<month> (fr)".to_string(),
            pattern: vec![regex("la\\s+premi(?:è|e)re\\s+quinzaine\\s+d[e']\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let mut td = TimeData::new(TimeForm::Month(month));
                td.early_late = Some(super::EarlyLate::Early);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "deuxième quinzaine d'<month> (fr)".to_string(),
            pattern: vec![regex("la\\s+deuxi(?:è|e)me\\s+quinzaine\\s+d[e']\\s*(janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let month = match m.as_str() {
                    "janvier" => 1,
                    "février" | "fevrier" => 2,
                    "mars" => 3,
                    "avril" => 4,
                    "mai" => 5,
                    "juin" => 6,
                    "juillet" => 7,
                    "août" | "aout" => 8,
                    "septembre" => 9,
                    "octobre" => 10,
                    "novembre" => 11,
                    "décembre" | "decembre" => 12,
                    _ => return None,
                };
                let mut td = TimeData::new(TimeForm::Month(month));
                td.early_late = Some(super::EarlyLate::Late);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "à partir de <time> (fr)".to_string(),
            pattern: vec![regex("(à|a) partir (de|du)"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let mut td = match &nodes[1].token_data {
                    TokenData::Time(t) => t.clone(),
                    _ => return None,
                };
                td.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "à partir du 8 (fr)".to_string(),
            pattern: vec![regex("(à|a) partir du\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                let mut td = TimeData::new(TimeForm::HourMinute(hour, 0, false));
                td.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "avant/jusqu'à <time> (fr)".to_string(),
            pattern: vec![regex("avant|jusqu[' ](à|a)"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let mut td = match &nodes[1].token_data {
                    TokenData::Time(t) => t.clone(),
                    _ => return None,
                };
                td.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(td))
            }),
        },
    ]);
    rules
}
