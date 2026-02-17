use super::{Direction, IntervalDirection, PartOfDay, TimeData, TimeForm};
use crate::dimensions::numeral::helpers::numeral_data;
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

fn duration_data(td: &TokenData) -> Option<&crate::dimensions::duration::DurationData> {
    match td {
        TokenData::Duration(d) => Some(d),
        _ => None,
    }
}

fn time_data(td: &TokenData) -> Option<&TimeData> {
    match td {
        TokenData::Time(t) => Some(t),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (de)".to_string(),
            pattern: vec![regex("jetzt|sofort|gerade eben|zu dieser zeit")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (de)".to_string(),
            pattern: vec![regex("heute")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (de)".to_string(),
            pattern: vec![regex("morgen")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (de)".to_string(),
            pattern: vec![regex("gestern")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "morning (de)".to_string(),
            pattern: vec![regex("am\\s+morgen|morgens?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))))),
        },
        Rule {
            name: "afternoon (de)".to_string(),
            pattern: vec![regex("am\\s+nachmittag|nachmittags?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))))),
        },
        Rule {
            name: "evening (de)".to_string(),
            pattern: vec![regex("am\\s+abend|abends?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))))),
        },
        Rule {
            name: "night (de)".to_string(),
            pattern: vec![regex("in\\s+der\\s+nacht|nachts?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Night))))),
        },
        Rule {
            name: "day of week (de)".to_string(),
            pattern: vec![regex("montags?|mo\\.?|die?nstags?|di\\.?|mittwochs?|mi\\.?|donn?erstags?|do\\.?|freitags?|fr\\.?|samstags?|sonnabends?|sa\\.?|sonntags?|so\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("montag") || s == "mo" || s == "mo." {
                    0
                } else if s.starts_with("dienstag") || s.starts_with("dienstags") || s == "di" || s == "di." {
                    1
                } else if s.starts_with("mittwoch") || s == "mi" || s == "mi." {
                    2
                } else if s.starts_with("donnerstag") || s.starts_with("donnerstags") || s == "do" || s == "do." {
                    3
                } else if s.starts_with("freitag") || s.starts_with("freitags") || s == "fr" || s == "fr." {
                    4
                } else if s.starts_with("samstag") || s.starts_with("samstags") || s.starts_with("sonnabend") || s == "sa" || s == "sa." {
                    5
                } else if s.starts_with("sonntag") || s.starts_with("sonntags") || s == "so" || s == "so." {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "1 märz (de)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*m(ä|ae)rz")],
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
            name: "erster märz (de)".to_string(),
            pattern: vec![regex("erste[rn]?\\s+m(ä|ae)rz")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "märz 3 (de)".to_string(),
            pattern: vec![regex("m(ä|ae)rz\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
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
            name: "am 15ten (de)".to_string(),
            pattern: vec![regex("am\\s+(\\d{1,2})ten")],
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
            name: "15. februar (de)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.\\s*februar")],
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
            name: "februar 15 (de)".to_string(),
            pattern: vec![regex("februar\\s*(\\d{1,2})")],
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
            name: "15te februar (de)".to_string(),
            pattern: vec![regex("(\\d{1,2})te\\s+februar")],
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
            name: "15.2. (de)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.(\\d{1,2})\\.?")],
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
            name: "13.-15. Juli (de)".to_string(),
            pattern: vec![regex("(\\d{1,2})(\\.?|ter)?\\s*([-–]|bis)\\s*(\\d{1,2})(\\.?|ter)?\\s*(januar|februar|m(ä|ae)rz|april|mai|juni|juli|august|september|oktober|november|dezember)")],
            production: Box::new(|nodes| {
                let (d1, d2, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(4)?, rm.group(6)?.to_lowercase()),
                    _ => return None,
                };
                let from_day: u32 = d1.parse().ok()?;
                let to_day: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&from_day) || !(1..=31).contains(&to_day) || from_day > to_day {
                    return None;
                }
                let month = match m.as_str() {
                    "januar" => 1,
                    "februar" => 2,
                    "märz" | "maerz" => 3,
                    "april" => 4,
                    "mai" => 5,
                    "juni" => 6,
                    "juli" => 7,
                    "august" => 8,
                    "september" => 9,
                    "oktober" => 10,
                    "november" => 11,
                    "dezember" => 12,
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::DateMDY {
                    month,
                    day: from_day,
                    year: None,
                });
                let to = TimeData::new(TimeForm::DateMDY {
                    month,
                    day: to_day,
                    year: None,
                });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "Juli 13 - Juli 15 (de)".to_string(),
            pattern: vec![regex("(januar|februar|m(ä|ae)rz|april|mai|juni|juli|august|september|oktober|november|dezember)\\s*(\\d{1,2})\\s*[-–]\\s*(januar|februar|m(ä|ae)rz|april|mai|juni|juli|august|september|oktober|november|dezember)\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (m1, d1, m2, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(3)?, rm.group(4)?.to_lowercase(), rm.group(6)?),
                    _ => return None,
                };
                let from_day: u32 = d1.parse().ok()?;
                let to_day: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&from_day) || !(1..=31).contains(&to_day) {
                    return None;
                }
                let month_num = |m: &str| -> Option<u32> {
                    Some(match m {
                        "januar" => 1,
                        "februar" => 2,
                        "märz" | "maerz" => 3,
                        "april" => 4,
                        "mai" => 5,
                        "juni" => 6,
                        "juli" => 7,
                        "august" => 8,
                        "september" => 9,
                        "oktober" => 10,
                        "november" => 11,
                        "dezember" => 12,
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
            name: "30. Mai 1980 (de)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.?\\s*(januar|februar|m(ä|ae)rz|april|mai|juni|juli|august|september|oktober|november|dezember)\\s*(\\d{2,4})")],
            production: Box::new(|nodes| {
                let (d, m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?.to_lowercase(), rm.group(4)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                let mut year: i32 = y.parse().ok()?;
                if y.len() == 2 {
                    year = year.checked_add(if year >= 50 { 1900 } else { 2000 })?;
                }
                let month = match m.as_str() {
                    "januar" => 1,
                    "februar" => 2,
                    "märz" | "maerz" => 3,
                    "april" => 4,
                    "mai" => 5,
                    "juni" => 6,
                    "juli" => 7,
                    "august" => 8,
                    "september" => 9,
                    "oktober" => 10,
                    "november" => 11,
                    "dezember" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: Some(year),
                })))
            }),
        },
        Rule {
            name: "<ordinal> Dezember (de)".to_string(),
            pattern: vec![
                dim(DimensionKind::Ordinal),
                regex("(januar|februar|m(ä|ae)rz|april|mai|juni|juli|august|september|oktober|november|dezember)"),
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
                    "januar" => 1,
                    "februar" => 2,
                    "märz" | "maerz" => 3,
                    "april" => 4,
                    "mai" => 5,
                    "juni" => 6,
                    "juli" => 7,
                    "august" => 8,
                    "september" => 9,
                    "oktober" => 10,
                    "november" => 11,
                    "dezember" => 12,
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
            name: "Oktober 2014 (de)".to_string(),
            pattern: vec![regex("[Oo]ktober\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(10))),
                ))))
            }),
        },
        Rule {
            name: "nächsten märz (de)".to_string(),
            pattern: vec![regex("n(ä|ae)chsten\\s+m(ä|ae)rz")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "kommende woche (de)".to_string(),
            pattern: vec![regex("kommende woche")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "letzte woche (de)".to_string(),
            pattern: vec![regex("letzte woche")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "nächste woche (de)".to_string(),
            pattern: vec![regex("n(ä|ae)chste woche")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "nächstes quartal (de)".to_string(),
            pattern: vec![regex("n(ä|ae)chstes quartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "drittes quartal (de)".to_string(),
            pattern: vec![regex("dritt?es quartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "4tes quartal 2018 (de)".to_string(),
            pattern: vec![regex("4tes quartal\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(4, year))))
            }),
        },
        Rule {
            name: "letztes jahr (de)".to_string(),
            pattern: vec![regex("letztes jahr")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "nächstes jahr (de)".to_string(),
            pattern: vec![regex("n(ä|ae)chstes jahr")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "dritter tag im oktober (de)".to_string(),
            pattern: vec![regex("dritter tag im oktober")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 3, year: None })))),
        },
        Rule {
            name: "christmas (de)".to_string(),
            pattern: vec![regex("weihnacht(en|stag)?")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "christmas day".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "new year day (de)".to_string(),
            pattern: vec![regex("neujahr(s(tag)?)?")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "new year's day".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "new year eve (de)".to_string(),
            pattern: vec![regex("silvester")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "new year's eve".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "christmas eve (de)".to_string(),
            pattern: vec![regex("heilig(er)?\\s+abend")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "christmas eve".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "halloween (de)".to_string(),
            pattern: vec![regex("hall?owe?en?")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "halloween".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "valentinstag (de)".to_string(),
            pattern: vec![regex("valentin'?stag")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day: 14,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "schweizer bundesfeiertag (de)".to_string(),
            pattern: vec![regex("schweiz(er)? (bundes)?feiertag|bundes feiertag")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 8,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "tag der deutschen einheit (de)".to_string(),
            pattern: vec![regex("tag (der)? deutsc?hen? einheit")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 10,
                    day: 3,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "nationalfeiertag (de)".to_string(),
            pattern: vec![regex("((ö|o)sterreichischer?)? nationalfeiertag|national feiertag")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 10,
                    day: 26,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "allerheiligen (de)".to_string(),
            pattern: vec![regex("allerheiligen?|aller heiligen?")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 11,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "nikolaus (de)".to_string(),
            pattern: vec![regex("nikolaus(tag)?|nikolaus tag|nikolo")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 12,
                    day: 6,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "muttertag (de)".to_string(),
            pattern: vec![regex("mutt?ertag|mutt?er( tag)?")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "mother's day".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "vatertag (de)".to_string(),
            pattern: vec![regex("vatt?er( ?tag)?")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "father's day".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "jom kippur (de)".to_string(),
            pattern: vec![regex("jom\\s+kippur|yom\\s+kippur")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "yom kippur".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "jom haatzmaut (de)".to_string(),
            pattern: vec![regex("jom\\s+ha'?atzmaut|yom\\s+ha'?atzmaut")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "yom haatzmaut".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "jom hashoah (de)".to_string(),
            pattern: vec![regex("jom\\s+hashoah|yom\\s+hashoah")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "yom hashoah".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "pessach (de)".to_string(),
            pattern: vec![regex("passover|pessach|pesach|passa|passah|pascha")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "passover".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "sukkot (de)".to_string(),
            pattern: vec![regex("sukkot")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "sukkot".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "shavuot (de)".to_string(),
            pattern: vec![regex("shavuot|schawuot")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "shavuot".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "hanukkah (de)".to_string(),
            pattern: vec![regex("hanukkah|hanukah|chanukah")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "hanukkah".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "purim (de)".to_string(),
            pattern: vec![regex("shushan\\s+purim|purim")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let name = if s.contains("shushan") {
                    "shushan purim"
                } else {
                    "purim"
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    name.to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "computed holiday aliases (de)".to_string(),
            pattern: vec![regex("chinesisches (neujahr(sfest)?|fr(ü|u)hlingsfest)|lag baomer|lag laomer|simchat torah|tisha b[' ]?av|schmini azeret|schemini azeret|shemini atzeret|rosch ha-?schana(h)?|chanukka|laubh(ü|u)ttenfest|holocaust-?gedenktag|islamisches neujahr|aschura(-tag)?|tu bishvat|tu bischevat|jamat ul-vida|jumu'?atul-wida|aufstieg des propheten|die nachtreise|aufstieg in den himmel")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let name = if s.contains("chinesisches") {
                    "chinese new year"
                } else if s.starts_with("lag baomer") || s.starts_with("lag laomer") {
                    "lag baomer"
                } else if s.starts_with("simchat torah") {
                    "simchat torah"
                } else if s.starts_with("tisha") {
                    "tisha b'av"
                } else if s.starts_with("schmini") || s.starts_with("schemini") || s.starts_with("shemini") {
                    "shemini atzeret"
                } else if s.starts_with("rosch") {
                    "rosh hashanah"
                } else if s.starts_with("chanukka") {
                    "hanukkah"
                } else if s.starts_with("laubh") {
                    "sukkot"
                } else if s.starts_with("holocaust") {
                    "holocaust day"
                } else if s.starts_with("islamisches") {
                    "islamic new year"
                } else if s.starts_with("aschura") {
                    "ashura"
                } else if s.starts_with("tu ") {
                    "tu bishvat"
                } else if s.starts_with("jamat") || s.starts_with("jumu") {
                    "jumu'atul-wida"
                } else if s.starts_with("aufstieg") || s.starts_with("die nachtreise") {
                    "isra and mi'raj"
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    name.to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "islamic holiday aliases (de)".to_string(),
            pattern: vec![regex("maulid an-nab(ī|i)|mawlid al-nabawi|opferfest|bakr id|amun jadid|laylat al[- ]qadr|lailat al[- ]qadr|nacht der (bestimmung|allmacht)|id ul[- ]adha")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let name = if s.contains("maulid") || s.contains("mawlid") {
                    "mawlid"
                } else if s.contains("opferfest") || s.contains("bakr id") || s.contains("ul-adha") {
                    "eid al-adha"
                } else if s.contains("amun jadid") {
                    "islamic new year"
                } else if s.contains("laylat") || s.contains("lailat") || s.contains("nacht der") {
                    "laylat al-qadr"
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    name.to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "observance aliases (de)".to_string(),
            pattern: vec![regex("global youth service-?tag|gysd|stunde der erde|earth hour|wesakfest|vesak|vaisakha|buddha-?tag|buddha purnima|koningsdag|k(ö|o)nigstag")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let name = if s.contains("youth") || s == "gysd" {
                    "gysd"
                } else if s.contains("erde") || s.contains("earth hour") {
                    "earth hour"
                } else if s.contains("vesak") || s.contains("vaisakha") || s.contains("buddha") {
                    "vesak"
                } else if s.contains("koningsdag") || s.contains("königstag") || s.contains("konigstag") {
                    "koningsdag"
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    name.to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "indian/jewish alias bridge (de)".to_string(),
            pattern: vec![regex("guru (gobind|govind) singh (geburtstag|jayanti)|guru (ravidass|ravidas) (geburtstag|jayanti)|rabindranath? jayanti|parsi neujahr|jamshedi navroz|dhanatrayodashi|dhanteras|kali chaudas|choti diwali|chhoti diwali|deepavali|diwali|lakshmi puja|bhai dooj|chhath|dala puja|surya shashthi|maha navami|maha saptami|dussehra|vijayadashami|navaratri|durga puja|karva chauth|rakhi|mahavir jayanti|mahaveer janma kalyanak|maha shivaratri|saraswati jayanti|pongal|makara? sankranthi|makar sankranti|maghi|bogi pandigai|maattu pongal|kaanum pongal|kanni pongal|vaisakhi|baisakhi|vasakhi|vaishakhi|onam|thiru onam|thiruvonam|vasant panchami|basant panchami|holika dahan|kamudu pyre|krishna janmashtami|gokulashtami|holi|dhulandi|phagwah|pargat diwas|valmiki jayanti")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let name = if s.contains("gobind") || s.contains("govind") {
                    "guru gobind singh jayanti"
                } else if s.contains("ravidass") || s.contains("ravidas") {
                    "guru ravidass jayanti"
                } else if s.contains("rabindra") || s.contains("rabindranath") {
                    "rabindra jayanti"
                } else if s.contains("parsi") || s.contains("jamshedi") {
                    "parsi new year"
                } else if s.contains("dhanteras") || s.contains("dhanatrayodashi") {
                    "dhanteras"
                } else if s.contains("kali chaudas") || s.contains("choti diwali") || s.contains("chhoti diwali") {
                    "chhoti holi"
                } else if s.contains("deepavali") || s == "diwali" || s.contains("lakshmi puja") {
                    "diwali"
                } else if s.contains("bhai dooj") {
                    "bhai dooj"
                } else if s == "chhath" || s.contains("dala puja") || s.contains("surya shashthi") {
                    "chhath"
                } else if s.contains("maha navami") {
                    "maha navami"
                } else if s.contains("maha saptami") {
                    "maha saptami"
                } else if s.contains("dussehra") || s.contains("vijayadashami") {
                    "vijayadashami"
                } else if s == "navaratri" || s.contains("durga puja") {
                    "navaratri"
                } else if s.contains("karva chauth") {
                    "karva chauth"
                } else if s == "rakhi" {
                    "raksha bandhan"
                } else if s.contains("mahavir") || s.contains("mahaveer") {
                    "mahavir jayanti"
                } else if s.contains("shivaratri") {
                    "maha shivaratri"
                } else if s.contains("saraswati jayanti") {
                    "dayananda saraswati jayanti"
                } else if s == "pongal" || s.contains("sankran") || s == "maghi" {
                    "thai pongal"
                } else if s.contains("bogi") {
                    "boghi"
                } else if s.contains("maattu") {
                    "mattu pongal"
                } else if s.contains("kaanum") || s.contains("kanni") {
                    "kaanum pongal"
                } else if s.contains("vaisakhi") || s.contains("baisakhi") || s.contains("vasakhi") || s.contains("vaishakhi") {
                    "vaisakhi"
                } else if s == "onam" || s.contains("thiru onam") || s.contains("thiruvonam") {
                    "thiru onam"
                } else if s.contains("vasant panchami") || s.contains("basant panchami") {
                    "vasant panchami"
                } else if s.contains("holika dahan") || s.contains("kamudu pyre") {
                    "holika dahan"
                } else if s.contains("janmashtami") || s.contains("gokulashtami") {
                    "krishna janmashtami"
                } else if s == "holi" || s.contains("dhulandi") || s.contains("phagwah") {
                    "holi"
                } else if s.contains("pargat diwas") || s.contains("valmiki") {
                    "pargat diwas"
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    name.to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "christian computed aliases (de)".to_string(),
            pattern: vec![regex("(christi\\s+)?himmelfahrt(stag)?|asch(er|e)(tag|mittwoch)|ostersonntag|ostermontag|karfreitag|karsamstag|karsonnabend|gr(ü|u)ndonnerstag|palmsonntag|pfingsten|fronleichnam|corpus\\s+christi|allerheiligen")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let name = if s.contains("himmelfahrt") {
                    "ascension day"
                } else if s.contains("ascher") {
                    "ash wednesday"
                } else if s.contains("ostermontag") {
                    "easter monday"
                } else if s.contains("ostersonntag") {
                    "easter"
                } else if s.contains("karfreitag") {
                    "good friday"
                } else if s.contains("karsamstag") || s.contains("karsonnabend") {
                    "holy saturday"
                } else if s.contains("gründonnerstag") || s.contains("grundonnerstag") {
                    "holy thursday"
                } else if s.contains("palmsonntag") {
                    "palm sunday"
                } else if s.contains("pfingsten") {
                    "pentecost"
                } else if s.contains("fronleichnam") || s.contains("corpus christi") {
                    "corpus christi"
                } else if s.contains("allerheiligen") {
                    "all saints day"
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    name.to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "aschetag <year> (de)".to_string(),
            pattern: vec![regex("aschetag\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "ash wednesday".to_string(),
                    Some(year),
                ))))
            }),
        },
        Rule {
            name: "named holiday <year> (de)".to_string(),
            pattern: vec![regex("([\\p{L}'’\\-]+(?:\\s+[\\p{L}'’\\-]+){1,5})\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (name, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    name.to_lowercase(),
                    Some(year),
                ))))
            }),
        },
        Rule {
            name: "single-word holiday <year> (de)".to_string(),
            pattern: vec![regex("([\\p{L}'’\\-]{4,})\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (name, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    name.to_lowercase(),
                    Some(year),
                ))))
            }),
        },
        Rule {
            name: "um 3 (de)".to_string(),
            pattern: vec![regex("um\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, true))))
            }),
        },
        Rule {
            name: "3 uhr (de)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*uhr")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, true))))
            }),
        },
        Rule {
            name: "um drei (de)".to_string(),
            pattern: vec![regex("um\\s+(ein|eins|zwei|drei|vier|f(ü|u)nf|sechs|sieben|acht|neun|zehn|elf|zw(ö|o)lf)")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour = match w {
                    "ein" | "eins" => 1,
                    "zwei" => 2,
                    "drei" => 3,
                    "vier" => 4,
                    "fünf" | "funf" => 5,
                    "sechs" => 6,
                    "sieben" => 7,
                    "acht" => 8,
                    "neun" => 9,
                    "zehn" => 10,
                    "elf" => 11,
                    "zwölf" | "zwolf" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, true))))
            }),
        },
        Rule {
            name: "viertel nach drei Uhr (de)".to_string(),
            pattern: vec![regex("viertel\\s+nach\\s+drei\\s+uhr")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(3, 15, true))))),
        },
        Rule {
            name: "zwanzig nach 3 (de)".to_string(),
            pattern: vec![regex("zwanzig\\s+nach\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 20, true))))
            }),
        },
        Rule {
            name: "um halb 4 (de)".to_string(),
            pattern: vec![regex("um\\s+halb\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if !(1..=24).contains(&hour) {
                    return None;
                }
                let out_hour = if hour == 24 { 23 } else { hour.checked_sub(1)? };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_hour, 30, true))))
            }),
        },
        Rule {
            name: "viertel vor 12 (de)".to_string(),
            pattern: vec![regex("viertel\\s+vor\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if !(1..=24).contains(&hour) {
                    return None;
                }
                let out_hour = if hour == 24 { 23 } else { hour.checked_sub(1)? };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_hour, 45, true))))
            }),
        },
        Rule {
            name: "vor 7 tagen (de)".to_string(),
            pattern: vec![regex("vor\\s*(\\d{1,2})\\s+tagen")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let days: i32 = d.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Day, offset: days.checked_neg()? })))
            }),
        },
        Rule {
            name: "ende des tages/monats/jahres (de)".to_string(),
            pattern: vec![regex("ende (des|vom) (tag(es)?|monat(s)?|jahr(es)?)")],
            production: Box::new(|nodes| {
                let g = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?.to_lowercase(),
                    _ => return None,
                };
                let grain = if g.starts_with("tag") {
                    Grain::Day
                } else if g.starts_with("monat") {
                    Grain::Month
                } else if g.starts_with("jahr") {
                    Grain::Year
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset { grain, offset: 0 }),
                })))
            }),
        },
        Rule {
            name: "bis zum ende des tages/monats/jahres (de)".to_string(),
            pattern: vec![regex("bis (zum )?ende (des|vom) (tag(es)?|monat(s)?|jahr(es)?)")],
            production: Box::new(|nodes| {
                let g = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(3)?.to_lowercase(),
                    _ => return None,
                };
                let grain = if g.starts_with("tag") {
                    Grain::Day
                } else if g.starts_with("monat") {
                    Grain::Month
                } else if g.starts_with("jahr") {
                    Grain::Year
                } else {
                    return None;
                };
                let mut td = TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset { grain, offset: 0 }),
                });
                td.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "vor ende des tages/monats/jahres (de)".to_string(),
            pattern: vec![regex("(noch )?vor ende (des|vom) (tag(es)?|monat(s)?|jahr(es)?)")],
            production: Box::new(|nodes| {
                let g = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(3)?.to_lowercase(),
                    _ => return None,
                };
                let grain = if g.starts_with("tag") {
                    Grain::Day
                } else if g.starts_with("monat") {
                    Grain::Month
                } else if g.starts_with("jahr") {
                    Grain::Year
                } else {
                    return None;
                };
                let mut td = TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset { grain, offset: 0 }),
                });
                td.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "in <duration> (de)".to_string(),
            pattern: vec![regex("\\bin\\b"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let dur = duration_data(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: dur.value,
                    grain: dur.grain,
                })))
            }),
        },
        Rule {
            name: "vor <duration> (de)".to_string(),
            pattern: vec![regex("\\bvor\\b"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let dur = duration_data(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: dur.value.checked_neg()?,
                    grain: dur.grain,
                })))
            }),
        },
        Rule {
            name: "nach <duration> (de)".to_string(),
            pattern: vec![regex("\\bnach\\b"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let dur = duration_data(&nodes[1].token_data)?;
                let mut td = TimeData::new(TimeForm::RelativeGrain {
                    n: dur.value,
                    grain: dur.grain,
                });
                td.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "within <duration> (de)".to_string(),
            pattern: vec![regex("binnen|innerhalb( von)?"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let dur = duration_data(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::Now)),
                    Box::new(TimeData::new(TimeForm::RelativeGrain {
                        n: dur.value,
                        grain: dur.grain,
                    })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "<duration> nach <time> (de)".to_string(),
            pattern: vec![
                dim(DimensionKind::Duration),
                regex("nach"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let dur = duration_data(&nodes[0].token_data)?;
                let base = time_data(&nodes[2].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n: dur.value,
                    grain: dur.grain,
                    base: Box::new(base.clone()),
                })))
            }),
        },
        Rule {
            name: "vor einer woche (de)".to_string(),
            pattern: vec![regex("vor\\s+ein(er|e|em)?\\s+woche")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: crate::dimensions::time_grain::Grain::Week,
                    offset: -1,
                })))
            }),
        },
        Rule {
            name: "letzte <n> <cycle> (de)".to_string(),
            pattern: vec![regex("(letzten?|vergangenen?)\\s*(\\d{1,4})\\s*(sekunden?|minuten?|stunden?|tage?n?|wochen?|monate?n?|jahre?n?)")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let count: i64 = n.parse().ok()?;
                let grain = if unit.starts_with("sek") {
                    crate::dimensions::time_grain::Grain::Second
                } else if unit.starts_with("min") {
                    crate::dimensions::time_grain::Grain::Minute
                } else if unit.starts_with("st") {
                    crate::dimensions::time_grain::Grain::Hour
                } else if unit.starts_with("tag") || unit.starts_with("tage") {
                    crate::dimensions::time_grain::Grain::Day
                } else if unit.starts_with("woch") {
                    crate::dimensions::time_grain::Grain::Week
                } else if unit.starts_with("monat") {
                    crate::dimensions::time_grain::Grain::Month
                } else if unit.starts_with("jahr") {
                    crate::dimensions::time_grain::Grain::Year
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: count,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "letzte <integer> <cycle> (de)".to_string(),
            pattern: vec![
                regex("letzten?|vergangenen?"),
                dim(DimensionKind::Numeral),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?.value;
                if n < 1.0 || n.fract() != 0.0 {
                    return None;
                }
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: n as i64,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "nächste <n> <cycle> (de)".to_string(),
            pattern: vec![regex("(n(ä|ae)chsten?|kommenden?)\\s*(\\d{1,4})\\s*(sekunden?|minuten?|stunden?|tage?n?|wochen?|monate?n?|jahre?n?)")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(3)?, m.group(4)?.to_lowercase()),
                    _ => return None,
                };
                let count: i64 = n.parse().ok()?;
                let grain = if unit.starts_with("sek") {
                    crate::dimensions::time_grain::Grain::Second
                } else if unit.starts_with("min") {
                    crate::dimensions::time_grain::Grain::Minute
                } else if unit.starts_with("st") {
                    crate::dimensions::time_grain::Grain::Hour
                } else if unit.starts_with("tag") || unit.starts_with("tage") {
                    crate::dimensions::time_grain::Grain::Day
                } else if unit.starts_with("woch") {
                    crate::dimensions::time_grain::Grain::Week
                } else if unit.starts_with("monat") {
                    crate::dimensions::time_grain::Grain::Month
                } else if unit.starts_with("jahr") {
                    crate::dimensions::time_grain::Grain::Year
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: count,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "nächste <integer> <cycle> (de)".to_string(),
            pattern: vec![
                regex("n(ä|ae)chsten?|kommenden?"),
                dim(DimensionKind::Numeral),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?.value;
                if n < 1.0 || n.fract() != 0.0 {
                    return None;
                }
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: n as i64,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "vor zwei wochen (de)".to_string(),
            pattern: vec![regex("vor\\s+zwei\\s+wochen")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Week, offset: -2 })))),
        },
    ]);
    rules
}
