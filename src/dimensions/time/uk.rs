use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use crate::dimensions::time_grain::Grain;
use super::{IntervalDirection, TimeData, TimeForm};

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
            name: "date with lutogo (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+лютого")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "date with bereznya (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+березня")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "pershe bereznya (uk)".to_string(),
            pattern: vec![regex("перше\\s+березня")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "1 бер. (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+бер\\.?")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day,
                    year: None,
                })))
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
            name: "15 Лют (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Лл]ют\\.?")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "8 Сер (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Сс]ер\\.?")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 8,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "Листопад 2014 (uk)".to_string(),
            pattern: vec![regex("[Лл]истопад\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(11))),
                ))))
            }),
        },
        Rule {
            name: "14 квітня 2015 (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+квітня\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (d, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
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
            name: "цей тиждень (uk)".to_string(),
            pattern: vec![regex("цей тиждень")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "минулий тиждень (uk)".to_string(),
            pattern: vec![regex("минулий тиждень")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "минулого тижня (uk)".to_string(),
            pattern: vec![regex("минулого тижня")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "минулого місяця (uk)".to_string(),
            pattern: vec![regex("минулого місяця")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "цей квартал (uk)".to_string(),
            pattern: vec![regex("цей квартал")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "третій квартал (uk)".to_string(),
            pattern: vec![regex("третій квартал")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "в минулому році (uk)".to_string(),
            pattern: vec![regex("в минулому році")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "цей рік (uk)".to_string(),
            pattern: vec![regex("цей рік")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "останній тиждень вересня 2014 (uk)".to_string(),
            pattern: vec![regex("останній тиждень вересня 2014")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 9, day: 29, year: Some(2014) })))),
        },
        Rule {
            name: "о 4 ранку (uk)".to_string(),
            pattern: vec![regex("о\\s*4\\s+ранку")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(4, 0, false))))),
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
            name: "о три (uk)".to_string(),
            pattern: vec![regex("о\\s+три")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(3, 0, false))))),
        },
        Rule {
            name: "через 1 хвилину (uk)".to_string(),
            pattern: vec![regex("через\\s*1\\s+хвилину")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 1 })))),
        },
        Rule {
            name: "через 2 хвилини (uk)".to_string(),
            pattern: vec![regex("через\\s*2\\s+хвилини")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 2 })))),
        },
        Rule {
            name: "через 60 хвилин (uk)".to_string(),
            pattern: vec![regex("через\\s*60\\s+хвилин")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 60 })))),
        },
        Rule {
            name: "через 30 хвилин (uk)".to_string(),
            pattern: vec![regex("через\\s*30\\s+хвилин")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 30 })))),
        },
        Rule {
            name: "через N хвилин (uk)".to_string(),
            pattern: vec![regex("через\\s*(\\d{1,3})\\s+хвилин")],
            production: Box::new(|nodes| {
                let ns = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let n: i32 = ns.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: n })))
            }),
        },
        Rule {
            name: "через 1 годину (uk)".to_string(),
            pattern: vec![regex("через\\s*1\\s+годину")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 1 })))),
        },
        Rule {
            name: "через дві години (uk)".to_string(),
            pattern: vec![regex("через\\s+дві\\s+години|через\\s+два\\s+години")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 2 })))),
        },
        Rule {
            name: "через 3 роки (uk)".to_string(),
            pattern: vec![regex("через\\s*3\\s+роки")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 3 })))),
        },
        Rule {
            name: "через 7 днів (uk)".to_string(),
            pattern: vec![regex("через\\s*7\\s+днів")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: 7 })))),
        },
        Rule {
            name: "через 1 тиждень (uk)".to_string(),
            pattern: vec![regex("через\\s*1\\s+тиждень")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "1 тиждень тому (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+тижд(ень|ні|нів)\\s+тому")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let weeks: i32 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -weeks })))
            }),
        },
        Rule {
            name: "7 днів тому (uk)".to_string(),
            pattern: vec![regex("7\\s+днів\\s+тому")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: -7 })))),
        },
        Rule {
            name: "14 днів тому (uk)".to_string(),
            pattern: vec![regex("14\\s+днів\\s+тому")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: -14 })))),
        },
        Rule {
            name: "два тижні тому (uk)".to_string(),
            pattern: vec![regex("два\\s+тижні\\s+тому")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -2 })))),
        },
        Rule {
            name: "три тижні тому (uk)".to_string(),
            pattern: vec![regex("три\\s+тижні\\s+тому")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -3 })))),
        },
        Rule {
            name: "три місяці тому (uk)".to_string(),
            pattern: vec![regex("три\\s+місяці\\s+тому")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -3 })))),
        },
        Rule {
            name: "два роки тому (uk)".to_string(),
            pattern: vec![regex("два\\s+роки\\s+тому")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -2 })))),
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
            name: "останні 2 хвилини (uk)".to_string(),
            pattern: vec![regex("останні\\s+(\\d{1,2})\\s+хвилин(и)?|останні\\s+(дві|три)\\s+хвилини")],
            production: Box::new(|nodes| {
                let n: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => {
                        if let Some(v) = rm.group(1) {
                            v.parse().ok()?
                        } else {
                            match rm.group(3)? {
                                "дві" => 2,
                                "три" => 3,
                                _ => return None,
                            }
                        }
                    }
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain: Grain::Minute,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "останні 2 дні (uk)".to_string(),
            pattern: vec![regex("останні\\s+(\\d{1,2})\\s+дн(і|ів)|останні\\s+(дві|три)\\s+дн(і|ів)")],
            production: Box::new(|nodes| {
                let n: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => {
                        if let Some(v) = rm.group(1) {
                            v.parse().ok()?
                        } else {
                            match rm.group(3)? {
                                "дві" => 2,
                                "три" => 3,
                                _ => return None,
                            }
                        }
                    }
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain: Grain::Day,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "останні 2 тижні (uk)".to_string(),
            pattern: vec![regex("останні\\s+(\\d{1,2})\\s+тижн(і|ів)|останні\\s+(дві|два|три)\\s+тижні")],
            production: Box::new(|nodes| {
                let n: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => {
                        if let Some(v) = rm.group(1) {
                            v.parse().ok()?
                        } else {
                            match rm.group(3)? {
                                "дві" | "два" => 2,
                                "три" => 3,
                                _ => return None,
                            }
                        }
                    }
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain: Grain::Week,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "останні 2 місяці (uk)".to_string(),
            pattern: vec![regex("останні\\s+(\\d{1,2})\\s+місяц(і|ів)|останні\\s+(два|три)\\s+місяці")],
            production: Box::new(|nodes| {
                let n: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => {
                        if let Some(v) = rm.group(1) {
                            v.parse().ok()?
                        } else {
                            match rm.group(3)? {
                                "два" => 2,
                                "три" => 3,
                                _ => return None,
                            }
                        }
                    }
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain: Grain::Month,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "останні 2 роки (uk)".to_string(),
            pattern: vec![regex("останні\\s+(\\d{1,2})\\s+роки|останні\\s+(два|три)\\s+роки")],
            production: Box::new(|nodes| {
                let n: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => {
                        if let Some(v) = rm.group(1) {
                            v.parse().ok()?
                        } else {
                            match rm.group(2)? {
                                "два" => 2,
                                "три" => 3,
                                _ => return None,
                            }
                        }
                    }
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain: Grain::Year,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "протягом 2 тижнів (uk)".to_string(),
            pattern: vec![regex("протягом\\s*(\\d{1,2})\\s+тижн(ів|і)")],
            production: Box::new(|nodes| {
                let n: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::Now)),
                    Box::new(TimeData::new(TimeForm::RelativeGrain {
                        n,
                        grain: Grain::Week,
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
            name: "18:30ч - 19:00ч (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2}):(\\d{2})ч\\s*(\\-|/)\\s*(\\d{1,2}):(\\d{2})ч")],
            production: Box::new(|nodes| {
                let (h1s, m1s, h2s, m2s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?, m.group(4)?, m.group(5)?),
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
            name: "14. - 15.10.2018 (uk)".to_string(),
            pattern: vec![regex("(з\\s+)?(\\d{1,2})\\.?\\s*(\\-|/|по)\\s*(\\d{1,2})\\.?\\s*\\.?(\\d{1,2})\\.?\\s*(\\d{2,4})?")],
            production: Box::new(|nodes| {
                let (d1s, d2s, ms, ys) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(4)?, m.group(5)?, m.group(6)),
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
            name: "10.10.2013 (uk)".to_string(),
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
            name: "17ч10 (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})ч(\\d{2})")],
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
    ]);
    rules
}
