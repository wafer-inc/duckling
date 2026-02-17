use super::{Direction, PartOfDay, TimeData, TimeForm};
use crate::dimensions::numeral::helpers::{is_natural, numeral_data};
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

fn time_data(td: &TokenData) -> Option<&TimeData> {
    match td {
        TokenData::Time(d) => Some(d),
        _ => None,
    }
}

fn is_not_latent_time(td: &TokenData) -> bool {
    matches!(td, TokenData::Time(t) if !t.latent)
}

fn is_time_of_day(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Time(TimeData {
            form: TimeForm::Hour(..) | TimeForm::HourMinute(..) | TimeForm::HourMinuteSecond(..),
            ..
        })
    )
}

fn is_day_of_week(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Time(TimeData {
            form: TimeForm::DayOfWeek(..),
            ..
        })
    )
}

fn is_part_of_day(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Time(TimeData {
            form: TimeForm::PartOfDay(..),
            ..
        })
    )
}

fn pt_month_num(s: &str) -> Option<u32> {
    let t = s.trim().to_lowercase();
    if t.starts_with("jan") {
        Some(1)
    } else if t.starts_with("fev") {
        Some(2)
    } else if t.starts_with("mar") {
        Some(3)
    } else if t.starts_with("abr") {
        Some(4)
    } else if t.starts_with("mai") {
        Some(5)
    } else if t.starts_with("jun") {
        Some(6)
    } else if t.starts_with("jul") {
        Some(7)
    } else if t.starts_with("ago") {
        Some(8)
    } else if t.starts_with("set") {
        Some(9)
    } else if t.starts_with("out") {
        Some(10)
    } else if t.starts_with("nov") {
        Some(11)
    } else if t.starts_with("dez") {
        Some(12)
    } else {
        None
    }
}

fn pt_month_ordinal(s: &str) -> Option<u32> {
    let t = s.to_lowercase();
    match t.as_str() {
        "primeiro" => Some(1),
        "segundo" => Some(2),
        "terceiro" => Some(3),
        "quarto" => Some(4),
        "quinto" => Some(5),
        "sexto" => Some(6),
        "sétimo" | "setimo" => Some(7),
        "oitavo" => Some(8),
        "nono" => Some(9),
        "décimo" | "decimo" => Some(10),
        "décimo primeiro" | "decimo primeiro" => Some(11),
        "décimo segundo" | "decimo segundo" | "último" | "ultimo" => Some(12),
        _ => None,
    }
}

fn pt_quarter_ordinal(s: &str) -> Option<u32> {
    let t = s.to_lowercase();
    match t.as_str() {
        "primeiro" => Some(1),
        "segundo" => Some(2),
        "terceiro" => Some(3),
        "quarto" | "último" | "ultimo" => Some(4),
        _ => None,
    }
}

fn pt_ordinal_value(s: &str) -> Option<u32> {
    if let Ok(n) = s.parse::<u32>() {
        return Some(n);
    }
    pt_month_ordinal(s)
}

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
            name: "the day after tomorrow (pt)".to_string(),
            pattern: vec![regex("depois\\s+de\\s+amanh(ã|a)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))),
        },
        Rule {
            name: "the day before yesterday (pt)".to_string(),
            pattern: vec![regex("anteontem|antes\\s+de\\s+ontem")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))),
        },
        Rule {
            name: "this|next <day-of-week> (pt)".to_string(),
            pattern: vec![regex("es[ts][ae]|pr(ó|o)xim[ao]"), predicate(is_day_of_week)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Future);
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "<named-month> (pt)".to_string(),
            pattern: vec![regex("janeiro|jan\\.?|fevereiro|fev\\.?|mar[çc]o|mar\\.?|abril|abr\\.?|maio|mai\\.?|junho|jun\\.?|julho|jul\\.?|agosto|ago\\.?|setembro|set\\.?|outubro|out\\.?|novembro|nov\\.?|dezembro|dez\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let month = pt_month_num(s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "dia <day-of-month> de <named-month> (pt)".to_string(),
            pattern: vec![regex("dia"), regex("(\\d{1,2})"), regex("de|\\/"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let d = match &nodes[1].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month = match &nodes[3].token_data {
                    TokenData::Time(TimeData {
                        form: TimeForm::Month(m),
                        ..
                    }) => *m,
                    _ => return None,
                };
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "proximo <cycle> (pt)".to_string(),
            pattern: vec![regex("pr(ó|o)xim(o|a)s?"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: 1 })))
            }),
        },
        Rule {
            name: "este <cycle> (pt)".to_string(),
            pattern: vec![regex("(n?es[st](es?|as?))"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: 0 })))
            }),
        },
        Rule {
            name: "passados n <cycle> (pt)".to_string(),
            pattern: vec![regex("(passad|[úu]ltim)[ao]s?"), predicate(is_natural), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?.value as i64;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "proximas n <cycle> (pt)".to_string(),
            pattern: vec![regex("pr(ó|o)xim(o|a)s?"), predicate(is_natural), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?.value as i64;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "last <time> (pt)".to_string(),
            pattern: vec![regex("[úu]ltim[ao]s?"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Past);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "ultimo <time> (pt)".to_string(),
            pattern: vec![regex("[úu]ltim[oa]"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Past);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "in the <part-of-day> (pt)".to_string(),
            pattern: vec![regex("(de|pela|a|à)"), predicate(is_part_of_day)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Time(td) => {
                    let mut t = td.clone();
                    t.latent = false;
                    Some(TokenData::Time(t))
                }
                _ => None,
            }),
        },
        Rule {
            name: "afternoon (pt)".to_string(),
            pattern: vec![regex("tarde")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))))),
        },
        Rule {
            name: "evening (pt)".to_string(),
            pattern: vec![regex("noite")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))))),
        },
        Rule {
            name: "midnight (pt)".to_string(),
            pattern: vec![regex("meia\\s*noite")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(0, 0, false))))),
        },
        Rule {
            name: "noon (pt)".to_string(),
            pattern: vec![regex("meio[\\s\\-]?dia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))),
        },
        Rule {
            name: "day of month (1st) (pt)".to_string(),
            pattern: vec![regex("primeiro|um|1o")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(1))))),
        },
        Rule {
            name: "morning (pt)".to_string(),
            pattern: vec![regex("manh[ãa]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))))),
        },
        Rule {
            name: "<dim time> da madrugada (pt)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("(da|na|pela)\\s+madruga(da)?")],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t),
                    Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Night))),
                ))))
            }),
        },
        Rule {
            name: "<dim time> da manha (pt)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("(da|na|pela)\\s+manh[ãa]")],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t),
                    Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))),
                ))))
            }),
        },
        Rule {
            name: "<dim time> da tarde (pt)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("(da|na|pela)\\s+tarde")],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t),
                    Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))),
                ))))
            }),
        },
        Rule {
            name: "season #primavera (pt)".to_string(),
            pattern: vec![regex("primavera")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(0))))),
        },
        Rule {
            name: "season #outono (pt)".to_string(),
            pattern: vec![regex("outono")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(2))))),
        },
        Rule {
            name: "season (pt)".to_string(),
            pattern: vec![regex("primavera|ver[ãa]o|outono|inverno")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.contains("primavera") {
                    0
                } else if s.contains("ver") {
                    1
                } else if s.contains("outono") {
                    2
                } else {
                    3
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "<part-of-day> dessa semana (pt)".to_string(),
            pattern: vec![predicate(is_part_of_day), regex("(d?es[ts]a semana)|agora")],
            production: Box::new(|nodes| {
                let p = time_data(&nodes[0].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(p),
                ))))
            }),
        },
        Rule {
            name: "Tiradentes (pt)".to_string(),
            pattern: vec![regex("tiradentes")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day: 21, year: None })))),
        },
        Rule {
            name: "Dia do trabalhador (pt)".to_string(),
            pattern: vec![regex("dia\\s+do\\s+trabalhador|dia\\s+do\\s+trabalho")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day: 1, year: None })))),
        },
        Rule {
            name: "Independecia (pt)".to_string(),
            pattern: vec![regex("independ[êe]ncia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 9, day: 7, year: None })))),
        },
        Rule {
            name: "Nossa Senhora Aparecida (pt)".to_string(),
            pattern: vec![regex("nossa\\s+senhora\\s+aparecida")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 12, year: None })))),
        },
        Rule {
            name: "Finados (pt)".to_string(),
            pattern: vec![regex("finados")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 11, day: 2, year: None })))),
        },
        Rule {
            name: "Proclamação da República (pt)".to_string(),
            pattern: vec![regex("proclama[çc][ãa]o\\s+da\\s+rep[úu]blica")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 11, day: 15, year: None })))),
        },
        Rule {
            name: "vespera de ano novo (pt)".to_string(),
            pattern: vec![regex("v[ée]spera\\s+de\\s+ano\\s+novo")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "new year's eve".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "ano novo (pt)".to_string(),
            pattern: vec![regex("ano\\s+novo")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "new year".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "week-end (pt)".to_string(),
            pattern: vec![regex("fim\\s+de\\s+semana|final\\s+de\\s+semana|week\\-?end")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "year (pt)".to_string(),
            pattern: vec![regex("\\b(1\\d{3}|20\\d{2}|2100)\\b")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        Rule {
            name: "year (latent) (pt)".to_string(),
            pattern: vec![regex("\\b(21\\d{2}|[3-9]\\d{3})\\b")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let mut td = TimeData::new(TimeForm::Year(year));
                td.latent = true;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "depois das <time-of-day> (pt)".to_string(),
            pattern: vec![regex("(depois|ap(ó|o)s) d?((a|á|à)[so]?|os?)"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.open_interval_direction = Some(super::IntervalDirection::After);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "<datetime> - <datetime> (interval) (pt)".to_string(),
            pattern: vec![predicate(is_not_latent_time), regex("\\-|até( ao?)?|ao?"), predicate(is_not_latent_time)],
            production: Box::new(|nodes| {
                let from = time_data(&nodes[0].token_data)?.clone();
                let to = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    true,
                ))))
            }),
        },
        Rule {
            name: "de <datetime> - <datetime> (interval) (pt)".to_string(),
            pattern: vec![regex("de?"), predicate(is_time_of_day), regex("\\-|até( ao?)?|ao?"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let from = time_data(&nodes[1].token_data)?.clone();
                let to = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    true,
                ))))
            }),
        },
        Rule {
            name: "hh(:|.|h)mm (time-of-day) (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))[:h\\.]([0-5]\\d)")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                let mm: u32 = m.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, mm, false))))
            }),
        },
        Rule {
            name: "time-of-day (latent) (pt)".to_string(),
            pattern: vec![regex("\\b([01]?\\d|2[0-3])\\b")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let mut td = TimeData::new(TimeForm::HourMinute(hour, 0, true));
                td.latent = true;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "dd[/-]mm (pt)".to_string(),
            pattern: vec![regex("(3[01]|[12]\\d|0?[1-9])[\\/\\-](0?[1-9]|1[0-2])")],
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
            name: "em <duration> (pt)".to_string(),
            pattern: vec![regex("em"), dim(DimensionKind::Duration)],
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
            name: "fazem <duration> (pt)".to_string(),
            pattern: vec![regex("faz(em)?"), dim(DimensionKind::Duration)],
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
            name: "<cycle> antes de <time> (pt)".to_string(),
            pattern: vec![dim(DimensionKind::TimeGrain), regex("antes d[eo]"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let grain = match &nodes[0].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::NthLastCycleOfTime {
                    n: 1,
                    grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "intersect by `da` or `de` (pt)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("d[ae]"), predicate(is_not_latent_time)],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?.clone();
                let t2 = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t1),
                    Box::new(t2),
                ))))
            }),
        },
        Rule {
            name: "intersect (pt)".to_string(),
            pattern: vec![predicate(is_not_latent_time), predicate(is_not_latent_time)],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?.clone();
                let t2 = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t1),
                    Box::new(t2),
                ))))
            }),
        },
        Rule {
            name: "two time tokens separated by \",\" (pt)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex(","), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?.clone();
                let t2 = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(Box::new(t1), Box::new(t2)))))
            }),
        },
        Rule {
            name: "this <time> (pt)".to_string(),
            pattern: vec![regex("es[ts][ae]"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Future);
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "n[ao] <date> (pt)".to_string(),
            pattern: vec![regex("n[ao]"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Time(td) => {
                    let mut t = td.clone();
                    t.latent = false;
                    Some(TokenData::Time(t))
                }
                _ => None,
            }),
        },
        Rule {
            name: "de <year> (pt)".to_string(),
            pattern: vec![regex("de"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Time(TimeData { form: TimeForm::Year(y), .. }) => {
                    Some(TokenData::Time(TimeData::new(TimeForm::Year(*y))))
                }
                _ => None,
            }),
        },
        Rule {
            name: "next <time> (pt)".to_string(),
            pattern: vec![regex("pr(ó|o)xim[oa]"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Future);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "<named-month|named-day> next (pt)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("pr(ó|o)xim[oa]|que\\s+vem")],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[0].token_data)?.clone();
                td.direction = Some(Direction::Future);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "<named-month|named-day> past (pt)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("passad[oa]|[úu]ltim[oa]|anterior")],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[0].token_data)?.clone();
                td.direction = Some(Direction::Past);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "n <cycle> (proximo|que vem) (pt)".to_string(),
            pattern: vec![predicate(is_natural), dim(DimensionKind::TimeGrain), regex("(pr(ó|o)xim[oa]s?|que\\s+vem)")],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "n <cycle> atras (pt)".to_string(),
            pattern: vec![predicate(is_natural), dim(DimensionKind::TimeGrain), regex("atr[aá]s")],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "n passados <cycle> (pt)".to_string(),
            pattern: vec![predicate(is_natural), regex("passad[oa]s?"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "n proximas <cycle> (pt)".to_string(),
            pattern: vec![predicate(is_natural), regex("pr(ó|o)xim[oa]s?"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "antes das <time-of-day> (pt)".to_string(),
            pattern: vec![regex("antes d?((a|á|à)[so]?|os?)"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.open_interval_direction = Some(super::IntervalDirection::Before);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "<time-of-day> am|pm (pt)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("(a\\.?m\\.?|p\\.?m\\.?)")],
            production: Box::new(|nodes| {
                let td = time_data(&nodes[0].token_data)?;
                let ampm = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                match td.form {
                    TimeForm::HourMinute(h, m, _) => {
                        let hh = if ampm.contains('p') && h < 12 {
                            h.checked_add(12)?
                        } else if ampm.contains('a') && h == 12 {
                            0
                        } else {
                            h
                        };
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, m, false))))
                    }
                    TimeForm::Hour(h, _) => {
                        let hh = if ampm.contains('p') && h < 12 {
                            h.checked_add(12)?
                        } else if ampm.contains('a') && h == 12 {
                            0
                        } else {
                            h
                        };
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, 0, false))))
                    }
                    _ => None,
                }
            }),
        },
        Rule {
            name: "<time-of-day> horas (pt)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("horas?")],
            production: Box::new(|nodes| match &nodes[0].token_data {
                TokenData::Time(td) => {
                    let mut t = td.clone();
                    t.latent = false;
                    Some(TokenData::Time(t))
                }
                _ => None,
            }),
        },
        Rule {
            name: "às <hour-min>(time-of-day) (pt)".to_string(),
            pattern: vec![regex("(à|a)s?"), predicate(is_time_of_day), regex("horas?")],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Time(td) => Some(TokenData::Time(td.clone())),
                _ => None,
            }),
        },
        Rule {
            name: "às <time-of-day> (pt)".to_string(),
            pattern: vec![regex("[àa]s"), predicate(is_time_of_day)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Time(td) => Some(TokenData::Time(td.clone())),
                _ => None,
            }),
        },
        Rule {
            name: "<hour-of-day> <integer> (as relative minutes) (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+(\\d{1,2})\\b")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                let mm: u32 = m.parse().ok()?;
                if hh > 23 || mm > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, mm, false))))
            }),
        },
        Rule {
            name: "<hour-of-day> and <relative minutes> (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+e\\s+(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                let mm: u32 = m.parse().ok()?;
                if hh > 23 || mm > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, mm, false))))
            }),
        },
        Rule {
            name: "<hour-of-day> <integer> (as relative minutes) minutos (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+(\\d{1,2})\\s+min\\.?(uto)?s?")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                let mm: u32 = m.parse().ok()?;
                if hh > 23 || mm > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, mm, false))))
            }),
        },
        Rule {
            name: "<hour-of-day> and <relative minutes> minutos (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+e\\s+(\\d{1,2})\\s+min\\.?(uto)?s?")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                let mm: u32 = m.parse().ok()?;
                if hh > 23 || mm > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, mm, false))))
            }),
        },
        Rule {
            name: "<hour-of-day> quarter (as relative minutes) (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+quinze")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, 15, false))))
            }),
        },
        Rule {
            name: "<hour-of-day> and quinze (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+e\\s+quinze")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, 15, false))))
            }),
        },
        Rule {
            name: "<hour-of-day> half (as relative minutes) (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+(meia|trinta)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, 30, false))))
            }),
        },
        Rule {
            name: "<hour-of-day> and half (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+e\\s+(meia|trinta)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, 30, false))))
            }),
        },
        Rule {
            name: "<hour-of-day> 3/4 (as relative minutes) (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+quarenta\\s+e\\s+cinco")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, 45, false))))
            }),
        },
        Rule {
            name: "<hour-of-day> and 3/4 (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+e\\s+quarenta\\s+e\\s+cinco")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, 45, false))))
            }),
        },
        Rule {
            name: "<integer> para as <hour-of-day> (as relative minutes) (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+para\\s+((o|a|à)s?)?\\s*((?:[01]?\\d)|(?:2[0-3]))")],
            production: Box::new(|nodes| {
                let (m, h) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(4)?),
                    _ => return None,
                };
                let mm: u32 = m.parse().ok()?;
                let hh: u32 = h.parse().ok()?;
                if hh > 23 || mm > 59 {
                    return None;
                }
                let out_h = if hh == 0 { 23 } else { hh.checked_sub(1)? };
                let out_m = 60_u32.checked_sub(mm)?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_h, out_m, false))))
            }),
        },
        Rule {
            name: "<integer> minutos para as <hour-of-day> (as relative minutes) (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+min\\.?(uto)?s?\\s+para\\s+((o|a|à)s?)?\\s*((?:[01]?\\d)|(?:2[0-3]))")],
            production: Box::new(|nodes| {
                let (m, h) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(5)?),
                    _ => return None,
                };
                let mm: u32 = m.parse().ok()?;
                let hh: u32 = h.parse().ok()?;
                if hh > 23 || mm > 59 {
                    return None;
                }
                let out_h = if hh == 0 { 23 } else { hh.checked_sub(1)? };
                let out_m = 60_u32.checked_sub(mm)?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_h, out_m, false))))
            }),
        },
        Rule {
            name: "quinze para as <hour-of-day> (as relative minutes) (pt)".to_string(),
            pattern: vec![regex("quinze\\s+para\\s+((o|a|à)s?)?\\s*((?:[01]?\\d)|(?:2[0-3]))")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(3)?,
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                if hh > 23 {
                    return None;
                }
                let out_h = if hh == 0 { 23 } else { hh.checked_sub(1)? };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_h, 45, false))))
            }),
        },
        Rule {
            name: "half para as <hour-of-day> (as relative minutes) (pt)".to_string(),
            pattern: vec![regex("(meia|trinta)\\s+para\\s+((o|a|à)s?)?\\s*((?:[01]?\\d)|(?:2[0-3]))")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(4)?,
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                if hh > 23 {
                    return None;
                }
                let out_h = if hh == 0 { 23 } else { hh.checked_sub(1)? };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_h, 30, false))))
            }),
        },
        Rule {
            name: "3/4 para as <hour-of-day> (as relative minutes) (pt)".to_string(),
            pattern: vec![regex("quarenta\\s+e\\s+cinco\\s+para\\s+((o|a|à)s?)?\\s*((?:[01]?\\d)|(?:2[0-3]))")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(3)?,
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                if hh > 23 {
                    return None;
                }
                let out_h = if hh == 0 { 23 } else { hh.checked_sub(1)? };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_h, 15, false))))
            }),
        },
        Rule {
            name: "<time-of-day> <part-of-day> (pt)".to_string(),
            pattern: vec![predicate(is_time_of_day), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?.clone();
                let p = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t),
                    Box::new(p),
                ))))
            }),
        },
        Rule {
            name: "<day-of-week> às <hour-min> (pt)".to_string(),
            pattern: vec![predicate(is_day_of_week), regex("[àa]s"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let d = time_data(&nodes[0].token_data)?.clone();
                let t = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(d),
                    Box::new(t),
                ))))
            }),
        },
        Rule {
            name: "dd[/-.]mm[/-.]yyyy (pt)".to_string(),
            pattern: vec![regex("(3[01]|[12]\\d|0?[1-9])[\\/.\\-](0?[1-9]|1[0-2])[\\/.\\-](\\d{2,4})")],
            production: Box::new(|nodes| {
                let (d, m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let mut year: i32 = y.parse().ok()?;
                if y.len() == 2 {
                    year = year.checked_add(if year < 50 { 2000 } else { 1900 })?;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "dd-dd <month>(interval) (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*(\\-|a|até)\\s*(\\d{1,2})\\s+de\\s+([[:alpha:]çãõáéíóú]+)")],
            production: Box::new(|nodes| {
                let (d1, d2, ms) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?, rm.group(4)?),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                let month = pt_month_num(ms)?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), true))))
            }),
        },
        Rule {
            name: "dd-dd <month> de (interval) (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*(\\-|a|até)\\s*(\\d{1,2})\\s+([[:alpha:]çãõáéíóú]+)\\s+de\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (d1, d2, ms, ys) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?, rm.group(4)?, rm.group(5)?),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                let month = pt_month_num(ms)?;
                let year: i32 = ys.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month, day: day1, year: Some(year) });
                let to = TimeData::new(TimeForm::DateMDY { month, day: day2, year: Some(year) });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), true))))
            }),
        },
        Rule {
            name: "<time> timezone (pt)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("\\b(YEKT|YEKST|YAKT|YAKST|WITA|WIT|WIB|WGT|WGST|WFT|WET|WEST|WAT|WAST|VUT|VLAT|VLAST|VET|UZT|UYT|UYST|UTC|ULAT|TVT|TMT|TLT|TKT|TJT|TFT|TAHT|SST|SRT|SGT|SCT|SBT|SAST|SAMT|RET|PYT|PYST|PWT|PST|PONT|PMST|PMDT|PKT|PHT|PHOT|PGT|PETT|PETST|PET|PDT|OMST|OMSST|NZST|NZDT|NUT|NST|NPT|NOVT|NOVST|NFT|NDT|NCT|MYT|MVT|MUT|MST|MSK|MSD|MMT|MHT|MDT|MAWT|MART|MAGT|MAGST|LINT|LHST|LHDT|KUYT|KST|KRAT|KRAST|KGT|JST|IST|IRST|IRKT|IRKST|IRDT|IOT|IDT|ICT|HOVT|HKT|GYT|GST|GMT|GILT|GFT|GET|GAMT|GALT|FNT|FKT|FKST|FJT|FJST|EST|EGT|EGST|EET|EEST|EDT|ECT|EAT|EAST|EASST|DAVT|ChST|CXT|CVT|CST|COT|CLT|CLST|CKT|CHAST|CHADT|CET|CEST|CDT|CCT|CAT|CAST|BTT|BST|BRT|BRST|BOT|BNT|AZT|AZST|AZOT|AZOST|AWST|AWDT|AST|ART|AQTT|ANAT|ANAST|AMT|AMST|ALMT|AKST|AKDT|AFT|AEST|AEDT|ADT|ACST|ACDT)\\b")],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                let tz = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_uppercase(),
                    _ => return None,
                };
                t.timezone = Some(tz);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<cycle> actual (pt)".to_string(),
            pattern: vec![dim(DimensionKind::TimeGrain), regex("ac?tual")],
            production: Box::new(|nodes| {
                let grain = match &nodes[0].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: 0 })))
            }),
        },
        Rule {
            name: "right now (pt)".to_string(),
            pattern: vec![regex("agora|j[áa]|(nesse|neste)\\s+instante")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "hhmm (military time-of-day) (pt)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))([0-5]\\d)")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                let mm: u32 = m.parse().ok()?;
                let mut td = TimeData::new(TimeForm::HourMinute(hh, mm, false));
                td.latent = true;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "dentro de <duration> (pt)".to_string(),
            pattern: vec![regex("(dentro\\s+de)|em"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let d = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::Now)),
                    Box::new(TimeData::new(TimeForm::RelativeGrain { n: d.value, grain: d.grain })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "<cycle> (que vem) (pt)".to_string(),
            pattern: vec![dim(DimensionKind::TimeGrain), regex("que\\s+vem|seguintes?")],
            production: Box::new(|nodes| {
                let grain = match &nodes[0].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: 1 })))
            }),
        },
        Rule {
            name: "n <cycle> passados (pt)".to_string(),
            pattern: vec![predicate(is_natural), dim(DimensionKind::TimeGrain), regex("(passad|[úu]ltim)[ao]s?|anterior(es)?")],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain { n, grain, past: true, interval: true })))
            }),
        },
        Rule {
            name: "dia <day-of-month> (non ordinal) (pt)".to_string(),
            pattern: vec![regex("dia"), regex("(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[1].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
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
            name: "yyyy-mm-dd (pt)".to_string(),
            pattern: vec![regex("(\\d{2,4})-(0?[1-9]|1[0-2])-(3[01]|[12]\\d|0?[1-9])")],
            production: Box::new(|nodes| {
                let (y, m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "this <part-of-day> (pt)".to_string(),
            pattern: vec![regex("es[ts]a"), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let p = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(p),
                ))))
            }),
        },
        Rule {
            name: "entre <datetime> e <datetime> (interval) (pt)".to_string(),
            pattern: vec![regex("entre(\\s+[ao])?|desde|(a\\s+partir\\s+)?d[eo]"), dim(DimensionKind::Time), regex("e|\\-|at[eé](\\s+ao?)?|ao?"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let from = time_data(&nodes[1].token_data)?.clone();
                let to = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), true))))
            }),
        },
        Rule {
            name: "<cycle> passado (pt)".to_string(),
            pattern: vec![dim(DimensionKind::TimeGrain), regex("passad[ao]s?|anterior(es)?")],
            production: Box::new(|nodes| {
                let grain = match &nodes[0].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: -1 })))
            }),
        },
        Rule {
            name: "passado <cycle> (pt)".to_string(),
            pattern: vec![regex("(passad|[úu]ltim)[ao]s?|anterior(es)?"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: -1 })))
            }),
        },
        Rule {
            name: "às <hour-min> <time-of-day> (pt)".to_string(),
            pattern: vec![regex("[àa]s"), predicate(is_time_of_day), regex("da"), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?.clone();
                let p = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(Box::new(t), Box::new(p)))))
            }),
        },
        Rule {
            name: "amanhã pela <part-of-day> (pt)".to_string(),
            pattern: vec![regex("amanh[ãa]"), regex("(da|na|pela|a)"), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let p = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Tomorrow)),
                    Box::new(p),
                ))))
            }),
        },
        Rule {
            name: "<day-of-month> (ordinal or number) de <named-month> (pt)".to_string(),
            pattern: vec![regex("([0-9]{1,2}|primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)"), regex("de|\\/"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let dom = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => pt_ordinal_value(rm.group(1)?)?,
                    _ => return None,
                };
                let month = match &nodes[2].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                if !(1..=31).contains(&dom) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day: dom, year: None })))
            }),
        },
        Rule {
            name: "<day-of-month> (ordinal or number) <named-month> (pt)".to_string(),
            pattern: vec![regex("([0-9]{1,2}|primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let dom = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => pt_ordinal_value(rm.group(1)?)?,
                    _ => return None,
                };
                let month = match &nodes[1].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                if !(1..=31).contains(&dom) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day: dom, year: None })))
            }),
        },
        Rule {
            name: "<named-month> <day-of-month> (pt)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("(\\d{1,2})")],
            production: Box::new(|nodes| {
                let month = match &nodes[0].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                let day = match &nodes[1].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.parse::<u32>().ok()?,
                    _ => return None,
                };
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "entre dd et dd <month>(interval) (pt)".to_string(),
            pattern: vec![regex("entre"), regex("(0?[1-9]|[12]\\d|3[01])"), regex("e?"), regex("(0?[1-9]|[12]\\d|3[01])"), regex("de"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let day1 = match &nodes[1].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.parse::<u32>().ok()?,
                    _ => return None,
                };
                let day2 = match &nodes[3].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.parse::<u32>().ok()?,
                    _ => return None,
                };
                let month = match &nodes[5].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::DateMDY { month, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), true))))
            }),
        },
        Rule {
            name: "<ordinal> <cycle> de <time> (pt)".to_string(),
            pattern: vec![regex("([0-9]{1,2}|primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)"), dim(DimensionKind::TimeGrain), regex("d[eo]|em"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (pt_ordinal_value(rm.group(1)?)? as i32).checked_sub(1)?,
                    _ => return None,
                };
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime { n, grain, base: Box::new(base) })))
            }),
        },
        Rule {
            name: "<ordinal> <cycle> <time> (pt)".to_string(),
            pattern: vec![regex("([0-9]{1,2}|primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)"), dim(DimensionKind::TimeGrain), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (pt_ordinal_value(rm.group(1)?)? as i32).checked_sub(1)?,
                    _ => return None,
                };
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime { n, grain, base: Box::new(base) })))
            }),
        },
        Rule {
            name: "o <ordinal> <cycle> de <time> (pt)".to_string(),
            pattern: vec![regex("o|a"), regex("([0-9]{1,2}|primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)"), dim(DimensionKind::TimeGrain), regex("d[eo]|em"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let n = match &nodes[1].token_data {
                    TokenData::RegexMatch(rm) => (pt_ordinal_value(rm.group(1)?)? as i32).checked_sub(1)?,
                    _ => return None,
                };
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[4].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime { n, grain, base: Box::new(base) })))
            }),
        },
        Rule {
            name: "o <ordinal> trimestre (pt)".to_string(),
            pattern: vec![regex("o|a"), regex("(primeiro|segundo|terceiro|quarto|[úu]ltimo)"), regex("trimestre")],
            production: Box::new(|nodes| {
                let q = match &nodes[1].token_data {
                    TokenData::RegexMatch(rm) => pt_quarter_ordinal(rm.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Quarter(q))))
            }),
        },
        Rule {
            name: "<ordinal> trimestre <year> (pt)".to_string(),
            pattern: vec![regex("(primeiro|segundo|terceiro|quarto|[úu]ltimo)"), regex("trimestre"), regex("(\\d{4})")],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => pt_quarter_ordinal(rm.group(1)?)?,
                    _ => return None,
                };
                let year = match &nodes[2].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.parse::<i32>().ok()?,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(q, year))))
            }),
        },
        Rule {
            name: "último <cycle> de <time> (pt)".to_string(),
            pattern: vec![regex("[úu]ltim[ao]s?"), dim(DimensionKind::TimeGrain), regex("d[eo]|em"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::LastCycleOfTime { grain, base: Box::new(base) })))
            }),
        },
        Rule {
            name: "último <cycle> <time> (pt)".to_string(),
            pattern: vec![regex("[úu]ltim[ao]s?"), dim(DimensionKind::TimeGrain), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::LastCycleOfTime { grain, base: Box::new(base) })))
            }),
        },
        Rule {
            name: "desde <month> dd-dd de (interval) (pt)".to_string(),
            pattern: vec![regex("desde|a\\s+partir\\s+d[eo]"), regex("(\\d{1,2})"), regex("at[eé](\\s+ao?)?|ao?"), regex("(\\d{1,2})"), regex("d[eo]|em"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let day1 = match &nodes[1].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.parse::<u32>().ok()?,
                    _ => return None,
                };
                let day2 = match &nodes[3].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.parse::<u32>().ok()?,
                    _ => return None,
                };
                let month = match &nodes[5].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::DateMDY { month, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), true))))
            }),
        },
        Rule {
            name: "desde <month> dd-dd (interval) (pt)".to_string(),
            pattern: vec![regex("desde|a\\s+partir\\s+d[eo]"), regex("(\\d{1,2})"), regex("at[eé](\\s+ao?)?|ao?"), regex("(\\d{1,2})"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let day1 = match &nodes[1].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.parse::<u32>().ok()?,
                    _ => return None,
                };
                let day2 = match &nodes[3].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.parse::<u32>().ok()?,
                    _ => return None,
                };
                let month = match &nodes[4].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::DateMDY { month, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), true))))
            }),
        },
        Rule {
            name: "<ordinal> <cycle> de <year> - <ordinal> <cycle> de <year> (pt)".to_string(),
            pattern: vec![regex("([0-9]{1,2}|primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)"), dim(DimensionKind::TimeGrain), regex("d[eo]|em"), dim(DimensionKind::Time), regex("\\-|at[eé](\\s+ao?)?|ao?"), regex("([0-9]{1,2}|primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)"), dim(DimensionKind::TimeGrain), regex("d[eo]|em"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let n1 = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (pt_ordinal_value(rm.group(1)?)? as i32).checked_sub(1)?,
                    _ => return None,
                };
                let g1 = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let y1 = time_data(&nodes[3].token_data)?.clone();
                let n2 = match &nodes[5].token_data {
                    TokenData::RegexMatch(rm) => (pt_ordinal_value(rm.group(1)?)? as i32).checked_sub(1)?,
                    _ => return None,
                };
                let g2 = match &nodes[6].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let y2 = time_data(&nodes[8].token_data)?.clone();
                let from = TimeData::new(TimeForm::NthGrainOfTime { n: n1, grain: g1, base: Box::new(y1) });
                let to = TimeData::new(TimeForm::NthGrainOfTime { n: n2, grain: g2, base: Box::new(y2) });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), true))))
            }),
        },
        Rule {
            name: "<ordinal> <cycle> <year> - <ordinal> <cycle> <year> (pt)".to_string(),
            pattern: vec![regex("([0-9]{1,2}|primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)"), dim(DimensionKind::TimeGrain), dim(DimensionKind::Time), regex("\\-|at[eé](\\s+ao?)?|ao?"), regex("([0-9]{1,2}|primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)"), dim(DimensionKind::TimeGrain), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let n1 = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (pt_ordinal_value(rm.group(1)?)? as i32).checked_sub(1)?,
                    _ => return None,
                };
                let g1 = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let y1 = time_data(&nodes[2].token_data)?.clone();
                let n2 = match &nodes[4].token_data {
                    TokenData::RegexMatch(rm) => (pt_ordinal_value(rm.group(1)?)? as i32).checked_sub(1)?,
                    _ => return None,
                };
                let g2 = match &nodes[5].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let y2 = time_data(&nodes[6].token_data)?.clone();
                let from = TimeData::new(TimeForm::NthGrainOfTime { n: n1, grain: g1, base: Box::new(y1) });
                let to = TimeData::new(TimeForm::NthGrainOfTime { n: n2, grain: g2, base: Box::new(y2) });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), true))))
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
            name: "<ordinal> mês [de] [year] (pt)".to_string(),
            pattern: vec![regex("(primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)\\s+m[êe]s(\\s+de)?\\s*(\\d{4})?")],
            production: Box::new(|nodes| {
                let (ord, year_opt) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(5)),
                    _ => return None,
                };
                let month = pt_month_ordinal(ord)?;
                if let Some(y) = year_opt {
                    let year: i32 = y.parse().ok()?;
                    Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                        Box::new(TimeData::new(TimeForm::Year(year))),
                        Box::new(TimeData::new(TimeForm::Month(month))),
                    ))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
                }
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
            name: "<ordinal> trimestre [de] [year] (pt)".to_string(),
            pattern: vec![regex("(primeiro|segundo|terceiro|quarto|[úu]ltimo)\\s+trimestre(\\s+de)?\\s*(\\d{4})?")],
            production: Box::new(|nodes| {
                let (ord, year_opt) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)),
                    _ => return None,
                };
                let q = pt_quarter_ordinal(ord)?;
                if let Some(y) = year_opt {
                    let year: i32 = y.parse().ok()?;
                    Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(q, year))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::Quarter(q))))
                }
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
            name: "<ordinal> mês [de] <year> até <ordinal> mês [de] <year> (pt)".to_string(),
            pattern: vec![regex("((de|do|desde|a\\s+partir\\s+d[eo]|entre(\\s+o)?)\\s+)?(primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)\\s+m[êe]s(\\s+de)?\\s*(\\d{4})\\s*(a|ao|at[eé](\\s+ao)?|\\-|e)\\s*(o\\s+)?(primeiro|segundo|terceiro|quarto|quinto|sexto|s[eé]timo|oitavo|nono|d[eé]cimo(\\s+primeiro|\\s+segundo)?|[úu]ltimo)\\s+m[êe]s(\\s+de)?\\s*(\\d{4})?")],
            production: Box::new(|nodes| {
                let (o1, y1s, o2, y2s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(3)?, m.group(7)?, m.group(11)?, m.group(15)),
                    _ => return None,
                };
                let m1 = pt_month_ordinal(o1)?;
                let m2 = pt_month_ordinal(o2)?;
                let y1: i32 = y1s.parse().ok()?;
                let y2: i32 = y2s.unwrap_or(y1s).parse().ok()?;
                let from = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(y1))),
                    Box::new(TimeData::new(TimeForm::Month(m1))),
                ));
                let to = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(y2))),
                    Box::new(TimeData::new(TimeForm::Month(m2))),
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
