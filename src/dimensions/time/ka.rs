use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};
use super::{Direction, PartOfDay, TimeData, TimeForm};

fn is_dom_token(td: &TokenData) -> bool {
    match td {
        TokenData::Numeral(n) => {
            let v = n.value as i64;
            (1..=31).contains(&v) && (n.value - v as f64).abs() < f64::EPSILON
        }
        TokenData::Ordinal(o) => (1..=31).contains(&o.value),
        _ => false,
    }
}

fn dom_value(td: &TokenData) -> Option<u32> {
    match td {
        TokenData::Numeral(n) if (n.value - n.value.floor()).abs() < f64::EPSILON => {
            let v = n.value as i64;
            if (1..=31).contains(&v) { Some(v as u32) } else { None }
        }
        TokenData::Ordinal(o) if (1..=31).contains(&o.value) => Some(o.value as u32),
        _ => None,
    }
}

fn ka_month_from_text(s: &str) -> Option<u32> {
    let t = s.to_lowercase();
    if t.starts_with("იანვ") {
        Some(1)
    } else if t.starts_with("თებერვ") {
        Some(2)
    } else if t.starts_with("მარტ") {
        Some(3)
    } else if t.starts_with("აპრილ") {
        Some(4)
    } else if t.starts_with("მაის") {
        Some(5)
    } else if t.starts_with("ივნის") {
        Some(6)
    } else if t.starts_with("ივლის") {
        Some(7)
    } else if t.starts_with("აგვისტ") {
        Some(8)
    } else if t.starts_with("სექტემბ") {
        Some(9)
    } else if t.starts_with("ოქტომბ") {
        Some(10)
    } else if t.starts_with("ნოემბ") {
        Some(11)
    } else if t.starts_with("დეკემბ") {
        Some(12)
    } else {
        None
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule { name: "now (ka)".to_string(), pattern: vec![regex("ახლა|ეხლა")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))) },
        Rule { name: "today (ka)".to_string(), pattern: vec![regex("დღეს")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))) },
        Rule { name: "tomorrow (ka)".to_string(), pattern: vec![regex("ხვალ")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))) },
        Rule { name: "yesterday (ka)".to_string(), pattern: vec![regex("გუშინ")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))) },
        Rule { name: "month may (ka)".to_string(), pattern: vec![regex("მაის(ი|ში)?")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(5))))) },
        Rule { name: "month feb (ka)".to_string(), pattern: vec![regex("თებერვალ(ი|ს)")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(2))))) },
        Rule {
            name: "named month (ka)".to_string(),
            pattern: vec![regex("იანვარ(ი|ში|ს)|თებერვალ(ი|ში|ს)|მარტ(ი|ში|ს)|აპრილ(ი|ში|ს)|მაის(ი|ში|ს)|ივნის(ი|ში|ს)|ივლის(ი|ში|ს)|აგვისტო|სექტემბ(ერი|ერში|ერს)|ოქტომბ(ერი|ერში|ერს)|ნოემბ(ერი|ერში|ერს)|დეკემბ(ერი|ერში|ერს)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let month = if s.starts_with("იანვარ") {
                    1
                } else if s.starts_with("თებერვალ") {
                    2
                } else if s.starts_with("მარტ") {
                    3
                } else if s.starts_with("აპრილ") {
                    4
                } else if s.starts_with("მაის") {
                    5
                } else if s.starts_with("ივნის") {
                    6
                } else if s.starts_with("ივლის") {
                    7
                } else if s.starts_with("აგვისტ") {
                    8
                } else if s.starts_with("სექტემბ") {
                    9
                } else if s.starts_with("ოქტომბ") {
                    10
                } else if s.starts_with("ნოემბ") {
                    11
                } else if s.starts_with("დეკემბ") {
                    12
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "day of week (ka)".to_string(),
            pattern: vec![regex("ორშაბათი?ს?|სამშაბათი?ს?|ოთხშაბათი?ს?|ხუთშაბათი?ს?|პარასკევი?ს?|შაბათი?ს?|კვირას?|კვირის")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.contains("ორშაბათ") {
                    0
                } else if s.contains("სამშაბათ") {
                    1
                } else if s.contains("ოთხშაბათ") {
                    2
                } else if s.contains("ხუთშაბათ") {
                    3
                } else if s.contains("პარასკევ") {
                    4
                } else if s.contains("შაბათ") {
                    5
                } else if s.contains("კვირა") || s.contains("კვირის") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "year with -ში (ka)".to_string(),
            pattern: vec![regex("(\\d{4})-?ში")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        Rule {
            name: "<dom> feb (ka)".to_string(),
            pattern: vec![predicate(is_dom_token), regex("თებერვალ(ი|ს)")],
            production: Box::new(|nodes| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: dom_value(&nodes[0].token_data)?, year: None })))
            }),
        },
        Rule {
            name: "1-ლი მარტი (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})-?ლი\\s*მარტი?ს?")],
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
            name: "პირველი მარტი (ka)".to_string(),
            pattern: vec![regex("პირველი\\s*მარტი?ს?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "3 მარტი (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*მარტი?ს?")],
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
            name: "8 აგვისტო (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*აგვისტო?ს?")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 8, day, year: None })))
            }),
        },
        Rule {
            name: "2014 წლის ოქტომბერი (ka)".to_string(),
            pattern: vec![regex("(\\d{4})\\s+წლის\\s+ოქტომბერი")],
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
            name: "14 აპრილი 2015 (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+აპრილი\\s+(\\d{4})")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "2015 წლის 14 აპრილი (ka)".to_string(),
            pattern: vec![regex("(\\d{4})\\s+წლის\\s+(\\d{1,2})\\s+აპრილი")],
            production: Box::new(|nodes| {
                let (y, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "2015 წლის 14 აპრილს (ka)".to_string(),
            pattern: vec![regex("(\\d{4})\\s+წლის\\s+(\\d{1,2})\\s+აპრილს")],
            production: Box::new(|nodes| {
                let (y, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "შემდეგი მარტი (ka)".to_string(),
            pattern: vec![regex("შემდეგი\\s+მარტი")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "წინა თვე (ka)".to_string(),
            pattern: vec![regex("წინა\\s+თვე")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "შემდეგი თვე (ka)".to_string(),
            pattern: vec![regex("შემდეგი\\s+თვე")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "შემდეგ კვარტალში (ka)".to_string(),
            pattern: vec![regex("შემდეგ\\s+კვარტალში")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "წელს მეორე კვარტალში (ka)".to_string(),
            pattern: vec![regex("წელს\\s+(პირველ|მეორ|მესამ|მეოთხ)ე?\\s+კვარტალ(ში|შია)?")],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let quarter = if q.starts_with("პირველ") {
                    1
                } else if q.starts_with("მეორ") {
                    2
                } else if q.starts_with("მესამ") {
                    3
                } else {
                    4
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::AllGrain(crate::dimensions::time_grain::Grain::Year))),
                    Box::new(TimeData::new(TimeForm::Quarter(quarter))),
                ))))
            }),
        },
        Rule {
            name: "შარშან (ka)".to_string(),
            pattern: vec![regex("შარშან")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "წინა წელს (ka)".to_string(),
            pattern: vec![regex("წინა\\s+წელს")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "მომავალი წელი (ka)".to_string(),
            pattern: vec![regex("მომავალი\\s+წელი")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "შემდეგ წელს (ka)".to_string(),
            pattern: vec![regex("შემდეგ\\s+წელს")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "ოქტომბრის მესამე დღე (ka)".to_string(),
            pattern: vec![regex("ოქტომბრის\\s+მესამე\\s+დღე")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 3, year: None })))),
        },
        Rule {
            name: "ოქტომბრის მე-3 დღე (ka)".to_string(),
            pattern: vec![regex("ოქტომბრის\\s+მე-3\\s+დღე")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 3, year: None })))),
        },
        Rule {
            name: "15 საათზე (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*საათზე")],
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
            name: "4-ის 15 წუთზე (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})-ის\\s*(\\d{1,2})\\s*წუთზე")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?),
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if hour > 23 || minute > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "3 საათსა და 15 წუთზე (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*საათსა\\s*და\\s*(\\d{1,2})\\s*წუთზე")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?),
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if hour > 23 || minute > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "4-ის ნახევარზე (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})-ის\\s*ნახევარ(ზე|ი)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if !(1..=24).contains(&hour) {
                    return None;
                }
                let out_hour = if hour == 24 { 23 } else { hour - 1 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_hour, 30, false))))
            }),
        },
        Rule {
            name: "20 საათი (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*საათი")],
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
            name: "7 დღის წინ (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*დღის\\s*წინ")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let days: i32 = d.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Day, offset: -days })))
            }),
        },
        Rule {
            name: "შვიდი დღის წინ (ka)".to_string(),
            pattern: vec![regex("შვიდი\\s*დღის\\s*წინ")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Day, offset: -7 })))),
        },
        Rule {
            name: "თოთხმეტი დღის წინ (ka)".to_string(),
            pattern: vec![regex("თოთხმეტი\\s*დღის\\s*წინ")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Day, offset: -14 })))),
        },
        Rule {
            name: "სამი თვის წინ (ka)".to_string(),
            pattern: vec![regex("სამი\\s+თვის\\s+წინ")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Month, offset: -3 })))),
        },
        Rule {
            name: "3 თვის წინ (ka)".to_string(),
            pattern: vec![regex("3\\s+თვის\\s+წინ")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Month, offset: -3 })))),
        },
        Rule {
            name: "გასულ ორ თვეში (ka)".to_string(),
            pattern: vec![regex("(გასულ|წინა)\\s+(ორ|2)\\s+თვე(ში|ს)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                n: 2,
                grain: crate::dimensions::time_grain::Grain::Month,
                past: true,
                interval: true,
            })))),
        },
        Rule {
            name: "15 წუთში (ka)".to_string(),
            pattern: vec![regex("(\\d{1,3})\\s*წუთში")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mins: i32 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: crate::dimensions::time_grain::Grain::Minute,
                    offset: mins,
                })))
            }),
        },
        Rule {
            name: "ორი წლის წინ (ka)".to_string(),
            pattern: vec![regex("ორი\\s+წლის\\s+წინ|2\\s+წლის\\s+წინ")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Year, offset: -2 })))),
        },
        Rule {
            name: "ივლისი 13-15 (ka)".to_string(),
            pattern: vec![regex("ივლისი\\s*(\\d{1,2})\\s*[-–]\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (d1, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "13-15 ივლისი (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*[-–]\\s*(\\d{1,2})\\s*ივლისი")],
            production: Box::new(|nodes| {
                let (d1, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "this/next/last season (ka)".to_string(),
            pattern: vec![regex("(ამ|ეს|შემდეგი|მომავალი|წინა|ბოლო|გასული)\\s+(გაზაფხულ(ი|ზე|ს)?|ზაფხულ(ი|ზე|ს)?|შემოდგომ(ა|აზე|ას)?|ზამთ(არი|არში|არს)?)")],
            production: Box::new(|nodes| {
                let (q, season_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.to_lowercase(), m.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let season = if season_s.contains("გაზაფხ") {
                    0
                } else if season_s.contains("ზაფხ") {
                    1
                } else if season_s.contains("შემოდგომ") {
                    2
                } else {
                    3
                };
                let mut t = TimeData::new(TimeForm::Season(season));
                if q == "შემდეგი" || q == "მომავალი" {
                    t.direction = Some(Direction::Future);
                } else if q == "წინა" || q == "ბოლო" || q == "გასული" {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "this/next/last season generic (ka)".to_string(),
            pattern: vec![regex("(ამ|ეს|შემდეგი|მომავალი|წინა|ბოლო|გასული)\\s+სეზონ(ი|ზე|ს)?")],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::Season(99));
                if q == "შემდეგი" || q == "მომავალი" {
                    t.direction = Some(Direction::Future);
                } else if q == "წინა" || q == "ბოლო" || q == "გასული" {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "this/next/last weekend (ka)".to_string(),
            pattern: vec![regex("(ამ|ეს|შემდეგი|მომავალი|წინა|ბოლო|გასულ)\\s+უქმე(ებზე|ზე|ები|ა)?")],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::Weekend);
                if q == "შემდეგი" || q == "მომავალი" {
                    t.direction = Some(Direction::Future);
                } else if q == "წინა" || q == "ბოლო" || q == "გასულ" {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "15 თებერვლის დილა (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+თებერვ(ალი|ალს|ლის)\\s+დილა(ს)?")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 2, day, year: None })),
                    Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))),
                ))))
            }),
        },
        Rule {
            name: "დღეს/ხვალ/გუშინ საღამოს (ka)".to_string(),
            pattern: vec![regex("(დღეს|ხვალ|გუშინ)?\\s*(საღამოს|ღამე|ღამით)")],
            production: Box::new(|nodes| {
                let day = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).unwrap_or("დღეს"),
                    _ => return None,
                };
                let day_td = if day == "ხვალ" {
                    TimeData::new(TimeForm::Tomorrow)
                } else if day == "გუშინ" {
                    TimeData::new(TimeForm::Yesterday)
                } else {
                    TimeData::new(TimeForm::Today)
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(day_td),
                    Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
                ))))
            }),
        },
        Rule {
            name: "ბოლო 2 წამი (ka)".to_string(),
            pattern: vec![regex("ბოლო\\s+(\\d{1,2})\\s+წამ(ი|ში)")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let secs: i64 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: secs,
                    grain: crate::dimensions::time_grain::Grain::Second,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "თვის ბოლო (ka)".to_string(),
            pattern: vec![regex("(ამ\\s+)?თვის\\s+ბოლო(სთვის|სკენ)?")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset {
                        grain: crate::dimensions::time_grain::Grain::Month,
                        offset: 0,
                    }),
                })))
            }),
        },
        Rule {
            name: "წლის დასაწყისი/შუა/ბოლო (ka)".to_string(),
            pattern: vec![regex("წლის\\s+(დასაწყისი|შუა|ბოლო)")],
            production: Box::new(|nodes| {
                let pos = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                if pos == "შუა" {
                    let mut t = TimeData::new(TimeForm::AllGrain(crate::dimensions::time_grain::Grain::Year));
                    t.early_late = Some(super::EarlyLate::Mid);
                    return Some(TokenData::Time(t));
                }
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: pos == "დასაწყისი",
                    target: Box::new(TimeForm::AllGrain(crate::dimensions::time_grain::Grain::Year)),
                })))
            }),
        },
        Rule {
            name: "იანვრის დასაწყისი/შუა/ბოლო (ka)".to_string(),
            pattern: vec![regex("(იანვრის|თებერვლის|მარტის|აპრილის|მაისის|ივნისის|ივლისის|აგვისტოს|სექტემბრის|ოქტომბრის|ნოემბრის|დეკემბრის)\\s+(დასაწყისი|შუა|ბოლო)")],
            production: Box::new(|nodes| {
                let (m_s, pos) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let month = ka_month_from_text(m_s)?;
                if pos == "შუა" {
                    let mut t = TimeData::new(TimeForm::Month(month));
                    t.early_late = Some(super::EarlyLate::Mid);
                    return Some(TokenData::Time(t));
                }
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: pos == "დასაწყისი",
                    target: Box::new(TimeForm::Month(month)),
                })))
            }),
        },
    ]);
    rules
}
