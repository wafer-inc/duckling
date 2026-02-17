use crate::dimensions::numeral::helpers::{is_natural, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};
use crate::dimensions::time_grain::Grain;
use super::{Direction, IntervalDirection, PartOfDay, TimeData, TimeForm};

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

fn is_month(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Time(TimeData {
            form: TimeForm::Month(..),
            ..
        })
    )
}

fn uk_month_num(s: &str) -> Option<u32> {
    let t = s.to_lowercase();
    match t.as_str() {
        "січень" | "січня" | "січ" => Some(1),
        "лютий" | "лютого" | "лют" => Some(2),
        "березень" | "березня" | "бер" => Some(3),
        "квітень" | "квітня" | "квіт" => Some(4),
        "травень" | "травня" | "трав" => Some(5),
        "червень" | "червня" | "чер" => Some(6),
        "липень" | "липня" | "лип" => Some(7),
        "серпень" | "серпня" | "серп" | "сер" => Some(8),
        "вересень" | "вересня" | "верес" | "вер" => Some(9),
        "жовтень" | "жовтня" | "жовт" => Some(10),
        "листопад" | "листопада" | "лист" | "лис" => Some(11),
        "грудень" | "грудня" | "груд" | "гру" => Some(12),
        _ => None,
    }
}

fn uk_hour_word(s: &str) -> Option<u32> {
    match s.to_lowercase().as_str() {
        "один" | "одна" => Some(1),
        "два" | "дві" => Some(2),
        "три" => Some(3),
        "чотири" => Some(4),
        "п'ять" | "пять" => Some(5),
        "шість" | "шiсть" => Some(6),
        "сім" | "сiм" => Some(7),
        "вісім" | "вiсiм" => Some(8),
        "дев'ять" | "девять" => Some(9),
        "десять" => Some(10),
        "одинадцять" => Some(11),
        "дванадцять" => Some(12),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (uk)".to_string(),
            pattern: vec![regex("зараз")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (uk)".to_string(),
            pattern: vec![regex("сьогодні")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (uk)".to_string(),
            pattern: vec![regex("завтра")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (uk)".to_string(),
            pattern: vec![regex("вчора")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (uk)".to_string(),
            pattern: vec![regex("понеділ(ок|ка)|пн|вівтор(ок|ка)|вт|серед(а|у)|ср|четвер(га)?|чт|п'ятниц(я|і|ю)|пт|субот(а|и|у)|сб|неділ(я|і|ю)|нд")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("пн") || s.starts_with("понеділ") {
                    0
                } else if s.starts_with("вт") || s.starts_with("вівтор") {
                    1
                } else if s.starts_with("ср") || s.starts_with("серед") {
                    2
                } else if s.starts_with("чт") || s.starts_with("четвер") {
                    3
                } else if s.starts_with("пт") || s.starts_with("п'ятниц") {
                    4
                } else if s.starts_with("сб") || s.starts_with("субот") {
                    5
                } else if s.starts_with("нд") || s.starts_with("неділ") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "year (uk)".to_string(),
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
            name: "year (latent) (uk)".to_string(),
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
            name: "time-of-day (latent) (uk)".to_string(),
            pattern: vec![regex("\\b((?:[01]?\\d)|(?:2[0-3]))ч\\b")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let mut td = TimeData::new(TimeForm::HourMinute(hour, 0, false));
                td.latent = true;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "noon (uk)".to_string(),
            pattern: vec![regex("полудень|опівдні")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))),
        },
        Rule {
            name: "<ordinal> quarter (uk)".to_string(),
            pattern: vec![regex("(перш(ий|ого)|друг(ий|ого)|трет(ій|ього)|четверт(ий|ого))\\s+квартал")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let q = if s.starts_with("перш") { 1 } else if s.starts_with("друг") { 2 } else if s.starts_with("трет") { 3 } else { 4 };
                Some(TokenData::Time(TimeData::new(TimeForm::Quarter(q))))
            }),
        },
        Rule {
            name: "<ordinal> quarter <year> (uk)".to_string(),
            pattern: vec![regex("(перш(ий|ого)|друг(ий|ого)|трет(ій|ього)|четверт(ий|ого))\\s+квартал\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (ord, ys) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(5)?),
                    _ => return None,
                };
                let year: i32 = ys.parse().ok()?;
                let q = if ord.starts_with("перш") { 1 } else if ord.starts_with("друг") { 2 } else if ord.starts_with("трет") { 3 } else { 4 };
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(q, year))))
            }),
        },
        Rule {
            name: "this|next <day-of-week> (uk)".to_string(),
            pattern: vec![regex("(цей|ця|цього|цьому|наступний|наступна|наступної|наступну)"), predicate(is_day_of_week)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Future);
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "absorption of , after named day (uk)".to_string(),
            pattern: vec![predicate(is_day_of_week), regex(",")],
            production: Box::new(|nodes| Some(nodes[0].token_data.clone())),
        },
        Rule {
            name: "last <time> (uk)".to_string(),
            pattern: vec![regex("(в\\s+)?минул(ий|а|ого|ому|ої|у)"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Past);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "this <time> (uk)".to_string(),
            pattern: vec![regex("(цей|ця|цього|цьому|це|ці)"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Future);
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "on <date> (uk)".to_string(),
            pattern: vec![regex("(на|в)"), predicate(is_not_latent_time)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Time(td) => Some(TokenData::Time(td.clone())),
                _ => None,
            }),
        },
        Rule {
            name: "morning (uk)".to_string(),
            pattern: vec![regex("вранці|ранок|ранку")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))))),
        },
        Rule {
            name: "lunch (uk)".to_string(),
            pattern: vec![regex("обід|в\\s+обід")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Lunch))))),
        },
        Rule {
            name: "afternoon (uk)".to_string(),
            pattern: vec![regex("після\\s+обіду")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))))),
        },
        Rule {
            name: "evening (uk)".to_string(),
            pattern: vec![regex("увечері|ввечері|вечір")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))))),
        },
        Rule {
            name: "night (uk)".to_string(),
            pattern: vec![regex("вночі|ніч")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Night))))),
        },
        Rule {
            name: "this <part-of-day> (uk)".to_string(),
            pattern: vec![regex("(цей|ця|цього|цьому)"), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let p = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(p),
                ))))
            }),
        },
        Rule {
            name: "<time> <part-of-day> (uk)".to_string(),
            pattern: vec![dim(DimensionKind::Time), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?.clone();
                let p = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(Box::new(t), Box::new(p)))))
            }),
        },
        Rule {
            name: "<part-of-day> of <time> (uk)".to_string(),
            pattern: vec![predicate(is_part_of_day), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let p = time_data(&nodes[0].token_data)?.clone();
                let t = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(Box::new(t), Box::new(p)))))
            }),
        },
        Rule {
            name: "between <time-of-day> and <time-of-day> (interval) (uk)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("(і|до)"), predicate(is_time_of_day)],
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
            name: "<datetime> - <datetime> (interval) (uk)".to_string(),
            pattern: vec![predicate(is_not_latent_time), regex("\\-|до|по"), predicate(is_not_latent_time)],
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
            name: "<time-of-day> - <time-of-day> (interval) (uk)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("\\-|/"), predicate(is_time_of_day)],
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
            name: "at <time-of-day> (uk)".to_string(),
            pattern: vec![regex("о"), predicate(is_time_of_day)],
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
            name: "until <time-of-day> (uk)".to_string(),
            pattern: vec![regex("до"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[1].token_data)?.clone();
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "after <time-of-day> (uk)".to_string(),
            pattern: vec![regex("після"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[1].token_data)?.clone();
                t.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "intersect by ',' (uk)".to_string(),
            pattern: vec![predicate(is_not_latent_time), regex(","), predicate(is_not_latent_time)],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?.clone();
                let t2 = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(Box::new(t1), Box::new(t2)))))
            }),
        },
        Rule {
            name: "intersect by 'of', 'from', 's (uk)".to_string(),
            pattern: vec![predicate(is_not_latent_time), regex("на"), predicate(is_not_latent_time)],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?.clone();
                let t2 = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(Box::new(t1), Box::new(t2)))))
            }),
        },
        Rule {
            name: "intersect (uk)".to_string(),
            pattern: vec![predicate(is_not_latent_time), predicate(is_not_latent_time)],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?.clone();
                let t2 = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(Box::new(t1), Box::new(t2)))))
            }),
        },
        Rule {
            name: "week-end (uk)".to_string(),
            pattern: vec![regex("вихідн(і|ий)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "nth <time> after <time> (uk)".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex("після"),
                dim(DimensionKind::Ordinal),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let target = time_data(&nodes[0].token_data)?.clone();
                let ord = match &nodes[2].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::NthClosestToTime {
                    n: (ord - 1) as i32,
                    target: Box::new(target),
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "nth <time> of <time> (uk)".to_string(),
            pattern: vec![dim(DimensionKind::Time), dim(DimensionKind::Ordinal), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let td1 = time_data(&nodes[0].token_data)?.clone();
                let ord = match &nodes[1].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                let td2 = time_data(&nodes[2].token_data)?.clone();
                match td1.form {
                    TimeForm::DayOfWeek(dow) => Some(TokenData::Time(TimeData::new(TimeForm::NthDOWOfTime {
                        n: (ord - 1) as i32,
                        dow,
                        base: Box::new(td2),
                    }))),
                    _ => Some(TokenData::Time(TimeData::new(TimeForm::Composed(Box::new(td2), Box::new(td1))))),
                }
            }),
        },
        Rule {
            name: "<ordinal> <cycle> of <time> (uk)".to_string(),
            pattern: vec![
                dim(DimensionKind::Ordinal),
                dim(DimensionKind::TimeGrain),
                regex("в"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let ord = match &nodes[0].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime {
                    n: (ord - 1) as i32,
                    grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "<time> after next (uk)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("після\\s+наступн(ої|ого)")],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[0].token_data)?.clone();
                td.direction = Some(Direction::FarFuture);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "last <cycle> of <time> (uk)".to_string(),
            pattern: vec![regex("останн(ій|я|є|ього|ьому)"), dim(DimensionKind::TimeGrain), regex("в"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::LastCycleOfTime {
                    grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "last <day-of-week> of <time> (uk)".to_string(),
            pattern: vec![regex("останн(ій|я|є|ього|ьому)"), predicate(is_day_of_week), regex("в"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let dow = match &nodes[1].token_data {
                    TokenData::Time(TimeData { form: TimeForm::DayOfWeek(d), .. }) => *d,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::LastDOWOfTime {
                    dow,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "<duration> after <time> (uk)".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("після"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                let base = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n: d.value,
                    grain: d.grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "<duration> before <time> (uk)".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("перед"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                let base = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n: -d.value,
                    grain: d.grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "<time> timezone (uk)".to_string(),
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
            name: "<day> <named-month> (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+([Сс]іч(?:ень|ня)?\\.?|[Лл]ют(?:ий|ого)?\\.?|[Бб]ер(?:езень|езня)?\\.?|[Кк]віт(?:ень|ня)?\\.?|[Тт]рав(?:ень|ня)?\\.?|[Чч]ерв(?:ень|ня)?\\.?|[Лл]ип(?:ень|ня)?\\.?|[Сс]ерп(?:ень|ня)?\\.?|[Сс]ер\\.?|[Вв]ер(?:есень|есня)?\\.?|[Жж]овт(?:ень|ня)?\\.?|[Лл]истопад(?:а)?\\.?|[Гг]руд(?:ень|ня)?\\.?)")],
            production: Box::new(|nodes| {
                let (ds, ms) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let day: u32 = ds.parse().ok()?;
                let month = uk_month_num(ms.trim_matches('.'))?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "перше <named-month> (uk)".to_string(),
            pattern: vec![regex("перше\\s+([Сс]ічня|[Лл]ютого|[Бб]ерезня|[Кк]вітня|[Тт]равня|[Чч]ервня|[Лл]ипня|[Сс]ерпня|[Вв]ересня|[Жж]овтня|[Лл]истопада|[Гг]рудня)")],
            production: Box::new(|nodes| {
                let ms = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let month = uk_month_num(ms)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day: 1, year: None })))
            }),
        },
        Rule {
            name: "15.2 (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.(\\d{1,2})")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "<named-month> <year> (uk)".to_string(),
            pattern: vec![regex("([Сс]іч(?:ень|ня)?\\.?|[Лл]ют(?:ий|ого)?\\.?|[Бб]ер(?:езень|езня)?\\.?|[Кк]віт(?:ень|ня)?\\.?|[Тт]рав(?:ень|ня)?\\.?|[Чч]ерв(?:ень|ня)?\\.?|[Лл]ип(?:ень|ня)?\\.?|[Сс]ерп(?:ень|ня)?\\.?|[Сс]ер\\.?|[Вв]ер(?:есень|есня)?\\.?|[Жж]овт(?:ень|ня)?\\.?|[Лл]истопад(?:а)?\\.?|[Гг]руд(?:ень|ня)?\\.?)\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let (ms, ys) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let month = uk_month_num(ms.trim_matches('.'))?;
                let year: i32 = ys.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month))),
                ))))
            }),
        },
        Rule {
            name: "<day> <named-month> <year> (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+([Сс]ічня|[Лл]ютого|[Бб]ерезня|[Кк]вітня|[Тт]равня|[Чч]ервня|[Лл]ипня|[Сс]ерпня|[Вв]ересня|[Жж]овтня|[Лл]истопада|[Гг]рудня)\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (ds, ms, ys) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let day: u32 = ds.parse().ok()?;
                let month = uk_month_num(ms)?;
                let year: i32 = ys.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: Some(year),
                })))
            }),
        },
        Rule {
            name: "<day-of-month> (non ordinal) <named-month> (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})"), predicate(is_month)],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month = match &nodes[1].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "<named-month> <day-of-month> (non ordinal) (uk)".to_string(),
            pattern: vec![predicate(is_month), regex("(\\d{1,2})")],
            production: Box::new(|nodes| {
                let month = match &nodes[0].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                let d = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "<named-month> <day-of-month> (ordinal) (uk)".to_string(),
            pattern: vec![predicate(is_month), regex("(\\d{1,2})-?(й|го|ого|е)")],
            production: Box::new(|nodes| {
                let month = match &nodes[0].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                let d = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "<day-of-month> (ordinal) (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})-?(й|го|ого|е)")],
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
            name: "<day-of-month>(ordinal) <named-month> (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})-?(й|го|ого|е)"), predicate(is_month)],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month = match &nodes[1].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "<day-of-month>(ordinal) <named-month> year (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})-?(й|го|ого|е)"), predicate(is_month), regex("(\\d{4})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month = match &nodes[1].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                let y = match &nodes[2].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "this <cycle> (uk)".to_string(),
            pattern: vec![
                regex("(на\\s+|в\\s+)?(цьому|ця|цей|цього|це|ці)"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: 0,
                })))
            }),
        },
        Rule {
            name: "next <cycle> (uk)".to_string(),
            pattern: vec![
                regex("(в\\s+|на\\s+)?наступн(ий|а|у|ого|ій|ому)"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: 1,
                })))
            }),
        },
        Rule {
            name: "last <cycle> (uk)".to_string(),
            pattern: vec![
                regex("(в\\s+)?минул(ий|а|ого|ому|ої|у)"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: -1,
                })))
            }),
        },
        Rule {
            name: "третій квартал (uk)".to_string(),
            pattern: vec![regex("третій квартал")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "останній тиждень <month> <year> (uk)".to_string(),
            pattern: vec![regex("останній\\s+тиждень\\s+([Вв]ересня|[Сс]ічня|[Лл]ютого|[Бб]ерезня|[Кк]вітня|[Тт]равня|[Чч]ервня|[Лл]ипня|[Сс]ерпня|[Жж]овтня|[Лл]истопада|[Гг]рудня)\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (ms, ys) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let month = uk_month_num(ms)?;
                let year: i32 = ys.parse().ok()?;
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::LastCycleOfTime {
                    grain: Grain::Week,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "о <hour> ранку (uk)".to_string(),
            pattern: vec![regex("о\\s*(\\d{1,2})\\s+ранку")],
            production: Box::new(|nodes| {
                let hs = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let h: u32 = hs.parse().ok()?;
                if h == 0 || h > 12 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, 0, false))))
            }),
        },
        Rule {
            name: "о 3 (uk)".to_string(),
            pattern: vec![regex("о\\s*(\\d{1,2})")],
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
            name: "3 години (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+години")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let h: i32 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: h })))
            }),
        },
        Rule {
            name: "о <hour-word> (uk)".to_string(),
            pattern: vec![regex("о\\s+(один|одна|два|дві|три|чотири|п'ять|пять|шість|шiсть|сім|сiм|вісім|вiсiм|дев'ять|девять|десять|одинадцять|дванадцять)")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let h = uk_hour_word(w)?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, 0, false))))
            }),
        },
        Rule {
            name: "in <duration> (uk)".to_string(),
            pattern: vec![regex("через"), dim(DimensionKind::Duration)],
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
            name: "<duration> ago (uk)".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("тому")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: -d.value,
                    grain: d.grain,
                })))
            }),
        },
        Rule {
            name: "1 рік після різдва (uk)".to_string(),
            pattern: vec![regex("1\\s+р[іi]к\\s+після\\s+р[іi]здва")],
            production: Box::new(|_| {
                let base = TimeData::new(TimeForm::DateMDY { month: 1, day: 7, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n: 1,
                    grain: Grain::Year,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "це літо / ця зима (uk)".to_string(),
            pattern: vec![regex("це\\s+літо|ця\\s+зима|ця\\s+весна|ця\\s+осінь")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.contains("весна") {
                    0
                } else if s.contains("літо") {
                    1
                } else if s.contains("осін") {
                    2
                } else {
                    3
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "різдво (uk)".to_string(),
            pattern: vec![regex("р[іi]здво")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 1,
                day: 7,
                year: None,
            })))),
        },
        Rule {
            name: "Новий рік (uk)".to_string(),
            pattern: vec![regex("новий\\s+р[іi]к")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 1,
                day: 1,
                year: None,
            })))),
        },
        Rule {
            name: "в ці вихідні (uk)".to_string(),
            pattern: vec![regex("(в\\s+)?ц[іi]\\s+вих[іi]дн[іi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "last n <cycle> (uk)".to_string(),
            pattern: vec![
                regex("останні"),
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
            ],
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
            name: "next n <cycle> (uk)".to_string(),
            pattern: vec![
                regex("наступні"),
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
            ],
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
            name: "within <duration> (uk)".to_string(),
            pattern: vec![regex("протягом"), dim(DimensionKind::Duration)],
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
            name: "до кінця дня (uk)".to_string(),
            pattern: vec![regex("до\\s+кінця\\s+дня")],
            production: Box::new(|_| {
                let mut td = TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset {
                        grain: Grain::Day,
                        offset: 0,
                    }),
                });
                td.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "midnight|EOD|end of day (uk)".to_string(),
            pattern: vec![regex("північ|кінець\\s+дня")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(0, 0, false))))),
        },
        Rule {
            name: "до кінця місяця (uk)".to_string(),
            pattern: vec![regex("до\\s+кінця\\s+місяця")],
            production: Box::new(|_| {
                let mut td = TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset {
                        grain: Grain::Month,
                        offset: 0,
                    }),
                });
                td.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "після 14 годин (uk)".to_string(),
            pattern: vec![regex("після\\s*(\\d{1,2})\\s*(годин(и)?|ч)")],
            production: Box::new(|nodes| {
                let hs = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = hs.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                let mut td = TimeData::new(TimeForm::HourMinute(hour, 0, false));
                td.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "до 11 години (uk)".to_string(),
            pattern: vec![regex("до\\s*(\\d{1,2})\\s*годин(и)?(\\s+ранку)?")],
            production: Box::new(|nodes| {
                let hs = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = hs.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                let mut td = TimeData::new(TimeForm::HourMinute(hour, 0, false));
                td.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "<hh:mm[ч]> - <hh:mm[ч]> (uk)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))[:.]([0-5]\\d)(?:годин(и|і|а)?|ч)?\\s*(\\-|/|до)\\s*((?:[01]?\\d)|(?:2[0-3]))[:.]([0-5]\\d)(?:годин(и|і|а)?|ч)?")],
            production: Box::new(|nodes| {
                let (h1s, m1s, h2s, m2s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?, m.group(5)?, m.group(6)?),
                    _ => return None,
                };
                let h1: u32 = h1s.parse().ok()?;
                let m1: u32 = m1s.parse().ok()?;
                let h2: u32 = h2s.parse().ok()?;
                let m2: u32 = m2s.parse().ok()?;
                if h1 > 23 || h2 > 23 || m1 > 59 || m2 > 59 {
                    return None;
                }
                let from = TimeData::new(TimeForm::HourMinute(h1, m1, false));
                let to = TimeData::new(TimeForm::HourMinute(h2, m2, false));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "dd.(mm.)? - dd.mm.(yy[yy]?)? (interval) (uk)".to_string(),
            pattern: vec![regex("(?:з\\s+)?(\\d{1,2})\\.?\\s*(\\-|/|по)\\s*(\\d{1,2})\\.?\\s*\\.?(\\d{1,2})\\.?\\s*(\\d{2,4})?")],
            production: Box::new(|nodes| {
                let (d1s, d2s, ms, ys) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?, m.group(4)?, m.group(5)),
                    _ => return None,
                };
                let day1: u32 = d1s.parse().ok()?;
                let day2: u32 = d2s.parse().ok()?;
                let month: u32 = ms.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) || !(1..=12).contains(&month) {
                    return None;
                }
                let year_opt = ys.and_then(|y| {
                    if y.len() == 2 {
                        format!("20{}", y).parse::<i32>().ok()
                    } else {
                        y.parse::<i32>().ok()
                    }
                });
                let from = TimeData::new(TimeForm::DateMDY {
                    month,
                    day: day1,
                    year: year_opt,
                });
                let to = TimeData::new(TimeForm::DateMDY {
                    month,
                    day: day2,
                    year: year_opt,
                });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "<month> dd-dd (interval) (uk)".to_string(),
            pattern: vec![predicate(is_month), regex("(\\d{1,2})\\s*(\\-|/|до|по)\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let month = match &nodes[0].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                let (d1s, d2s) = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?),
                    _ => return None,
                };
                let day1: u32 = d1s.parse().ok()?;
                let day2: u32 = d2s.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), true))))
            }),
        },
        Rule {
            name: "yyyy-mm-dd (uk)".to_string(),
            pattern: vec![regex("(\\d{2,4})-(0?[1-9]|1[0-2])-(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (ys, ms, ds) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let year: i32 = ys.parse().ok()?;
                let month: u32 = ms.parse().ok()?;
                let day: u32 = ds.parse().ok()?;
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "dd.mm.yyyy (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.(\\d{1,2})\\.(\\d{4})")],
            production: Box::new(|nodes| {
                let (d, m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let year: i32 = y.parse().ok()?;
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: Some(year),
                })))
            }),
        },
        Rule {
            name: "mm/dd (uk)".to_string(),
            pattern: vec![regex("([012]?\\d|30|31)\\.(0?[1-9]|1[0-2])\\.?")],
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
            name: "mm/dd/yyyy (uk)".to_string(),
            pattern: vec![regex("([012]?\\d|30|31)\\.(0?[1-9]|1[0-2])\\.(\\d{2,4})")],
            production: Box::new(|nodes| {
                let (d, m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let mut year: i32 = y.parse().ok()?;
                if y.len() == 2 {
                    year += if year < 50 { 2000 } else { 1900 };
                }
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "hh:mm (uk)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))[:.]([0-5]\\d)")],
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
            name: "<time-of-day>  o'clock (uk)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("годин(а|и|і)?|ч(?:[\\s'\"\\-_{}\\[\\]()]|$)")],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                t.latent = false;
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "hhчmm (uk)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))ч([0-5]\\d)")],
            production: Box::new(|nodes| {
                let (hs, ms) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let h: u32 = hs.parse().ok()?;
                let m: u32 = ms.parse().ok()?;
                if h > 23 || m > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, m, false))))
            }),
        },
        Rule {
            name: "hhmm (military) (uk)".to_string(),
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
            name: "<hour-of-day> <integer> (as relative minutes) (uk)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+(\\d{1,2})\\s*хв(илин)?")],
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
    ]);
    rules
}
