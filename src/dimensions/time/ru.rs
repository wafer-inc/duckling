use crate::dimensions::time_grain::Grain;
use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{IntervalDirection, PartOfDay, TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ru)".to_string(),
            pattern: vec![regex("сейчас")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ru)".to_string(),
            pattern: vec![regex("сегодня")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ru)".to_string(),
            pattern: vec![regex("завтра")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ru)".to_string(),
            pattern: vec![regex("вчера")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (ru)".to_string(),
            pattern: vec![regex("понедельник(а)?|пн|вторник(а)?|вт|сред(а|у)|ср|четверг(а)?|чт|пятниц(а|у)|пт|суббот(а|у)|сб|воскресенье|вс|в\\s+пятницу")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.trim(),
                    _ => return None,
                };
                let dow = match s {
                    "понедельник" | "понедельника" | "пн" => 0,
                    "вторник" | "вторника" | "вт" => 1,
                    "среда" | "среду" | "ср" => 2,
                    "четверг" | "четверга" | "чт" => 3,
                    "пятница" | "пятницу" | "пт" | "в пятницу" => 4,
                    "суббота" | "субботу" | "сб" => 5,
                    "воскресенье" | "вс" => 6,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "эта неделя (ru)".to_string(),
            pattern: vec![regex("эта\\s+недел[яиюе]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "следующая неделя (ru)".to_string(),
            pattern: vec![regex("следующ[а-я]+\\s+недел[яиюе]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "этот месяц (ru)".to_string(),
            pattern: vec![regex("этот\\s+месяц")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Month))))),
        },
        Rule {
            name: "этот год (ru)".to_string(),
            pattern: vec![regex("этот\\s+год")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "прошлый месяц (ru)".to_string(),
            pattern: vec![regex("прошл[а-я]+\\s+месяц")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "следующий месяц (ru)".to_string(),
            pattern: vec![regex("следующ(ий|его|ем)\\s+месяц(е)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "следующий год (ru)".to_string(),
            pattern: vec![regex("следующ(ий|его|ем)\\s+год(у)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "этот квартал (ru)".to_string(),
            pattern: vec![regex("этот\\s+квартал")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "следующий квартал (ru)".to_string(),
            pattern: vec![regex("следующ(ий|его|ем)\\s+квартал(е)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "третий квартал (ru)".to_string(),
            pattern: vec![regex("трет(ий|ьего|ьем)\\s+квартал(е)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "четвертый квартал 2018 (ru)".to_string(),
            pattern: vec![regex("четв(е|ё)рт(ый|ого|ом)\\s+квартал\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(3)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(4, year))))
            }),
        },
        Rule {
            name: "в 4 утра (ru)".to_string(),
            pattern: vec![regex("в\\s*(\\d{1,2})\\s+утра|\\b(\\d{1,2}):(\\d{2})\\s+утра")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => {
                        if let Some(h1) = rm.group(1) {
                            (h1.to_string(), "0".to_string())
                        } else {
                            (rm.group(2)?.to_string(), rm.group(3)?.to_string())
                        }
                    }
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if hour == 0 || hour > 11 || minute > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "в полночь (ru)".to_string(),
            pattern: vec![regex("в\\s+полночь|полночь")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(0, 0, false))))),
        },
        Rule {
            name: "в полдень (ru)".to_string(),
            pattern: vec![regex("в\\s+полдень|полдень")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))),
        },
        Rule {
            name: "в 3 (ru)".to_string(),
            pattern: vec![regex("в\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
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
            name: "3 часа (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+час(а|ов)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour == 0 || hour > 23 {
                    return None;
                }
                let out = if hour < 12 { hour + 12 } else { hour };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out, 0, false))))
            }),
        },
        Rule {
            name: "в три (ru)".to_string(),
            pattern: vec![regex("в\\s+три")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))),
        },
        Rule {
            name: "в пятнадцать часов (ru)".to_string(),
            pattern: vec![regex("в\\s+пятнадцать\\s+часов")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))),
        },
        Rule {
            name: "через 1 секунду (ru)".to_string(),
            pattern: vec![regex("через\\s*(1\\s+секунду|секунду)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Second,
                offset: 1,
            })))),
        },
        Rule {
            name: "через 1 минуту (ru)".to_string(),
            pattern: vec![regex("через\\s*1\\s+минуту")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 1,
            })))),
        },
        Rule {
            name: "через 2 минуты (ru)".to_string(),
            pattern: vec![regex("через\\s*2\\s+минуты")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 2,
            })))),
        },
        Rule {
            name: "через 60 минут (ru)".to_string(),
            pattern: vec![regex("через\\s*60\\s+минут")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 60,
            })))),
        },
        Rule {
            name: "через N минут (ru)".to_string(),
            pattern: vec![regex("через\\s*(\\d{1,4})\\s+минут(у|ы)?")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let offset: i32 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: Grain::Minute,
                    offset,
                })))
            }),
        },
        Rule {
            name: "через 30 минут (ru)".to_string(),
            pattern: vec![regex("через\\s*30\\s+минут")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 30,
            })))),
        },
        Rule {
            name: "через час (ru)".to_string(),
            pattern: vec![regex("через\\s*(1\\s+час|один\\s+час|час)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Hour,
                offset: 1,
            })))),
        },
        Rule {
            name: "через два часа (ru)".to_string(),
            pattern: vec![regex("через\\s+два\\s+часа")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Hour,
                offset: 2,
            })))),
        },
        Rule {
            name: "через 24 часа (ru)".to_string(),
            pattern: vec![regex("через\\s*24\\s+часа")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Hour,
                offset: 24,
            })))),
        },
        Rule {
            name: "через 3 года (ru)".to_string(),
            pattern: vec![regex("через\\s*3\\s+года")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Year,
                offset: 3,
            })))),
        },
        Rule {
            name: "через 7 дней (ru)".to_string(),
            pattern: vec![regex("через\\s*7\\s+дней")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Day,
                offset: 7,
            })))),
        },
        Rule {
            name: "через неделю (ru)".to_string(),
            pattern: vec![regex("через\\s*(1\\s+неделю|неделю)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Week,
                offset: 1,
            })))),
        },
        Rule {
            name: "7 дней назад (ru)".to_string(),
            pattern: vec![regex("7\\s+дней(\\s+тому)?\\s+назад")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Day,
                offset: -7,
            })))),
        },
        Rule {
            name: "14 дней назад (ru)".to_string(),
            pattern: vec![regex("14\\s+дней\\s+назад")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Day,
                offset: -14,
            })))),
        },
        Rule {
            name: "две недели назад (ru)".to_string(),
            pattern: vec![regex("две\\s+недели\\s+назад")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Week,
                offset: -2,
            })))),
        },
        Rule {
            name: "неделю назад (ru)".to_string(),
            pattern: vec![regex("(1\\s+неделю\\s+назад|неделю\\s+тому\\s+назад)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Week,
                offset: -1,
            })))),
        },
        Rule {
            name: "три недели назад (ru)".to_string(),
            pattern: vec![regex("три\\s+недели\\s+назад")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Week,
                offset: -3,
            })))),
        },
        Rule {
            name: "три месяца назад (ru)".to_string(),
            pattern: vec![regex("три\\s+месяца\\s+назад")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Month,
                offset: -3,
            })))),
        },
        Rule {
            name: "два года назад (ru)".to_string(),
            pattern: vec![regex("два\\s+года\\s+назад")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Year,
                offset: -2,
            })))),
        },
        Rule {
            name: "лето (ru)".to_string(),
            pattern: vec![regex("лет(о|а)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(1))))),
        },
        Rule {
            name: "весна (ru)".to_string(),
            pattern: vec![regex("весна")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(0))))),
        },
        Rule {
            name: "зима (ru)".to_string(),
            pattern: vec![regex("зима")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(3))))),
        },
        Rule {
            name: "рождество (ru)".to_string(),
            pattern: vec![regex("рождество(\\s+христово)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 1,
                day: 7,
                year: None,
            })))),
        },
        Rule {
            name: "новый год (ru)".to_string(),
            pattern: vec![regex("новый\\s+год")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "new year".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "сегодня вечером (ru)".to_string(),
            pattern: vec![regex("сегодня\\s+в\\s+вечер(а|ом)|сегодня\\s+вечером")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Today)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "завтра вечером (ru)".to_string(),
            pattern: vec![regex("завтра\\s+вечером")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Tomorrow)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "завтра в обед (ru)".to_string(),
            pattern: vec![regex("завтра\\s+в\\s+обед")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Tomorrow)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Lunch))),
            ))))),
        },
        Rule {
            name: "вчера вечером (ru)".to_string(),
            pattern: vec![regex("вчера\\s+вечером")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Yesterday)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "в эти выходные (ru)".to_string(),
            pattern: vec![regex("в\\s+эти\\s+выходные")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "последние 2 секунды (ru)".to_string(),
            pattern: vec![regex("последние\\s+(2|две)\\s+секунды")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Second, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "следующие 3 секунды (ru)".to_string(),
            pattern: vec![regex("следующие\\s+(3|три)\\s+секунды")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Second, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "последние 2 минуты (ru)".to_string(),
            pattern: vec![regex("последние\\s+(2|две)\\s+минуты")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "следующие 3 минуты (ru)".to_string(),
            pattern: vec![regex("следующие\\s+(3|три)\\s+минуты")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "следующие 3 часа (ru)".to_string(),
            pattern: vec![regex("следующие\\s+(3|три)\\s+часа")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "последние 2 дня (ru)".to_string(),
            pattern: vec![regex("последние\\s+(2|два)\\s+дня")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "следующие 3 дня (ru)".to_string(),
            pattern: vec![regex("следующие\\s+(3|три)\\s+дня")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "последние 2 недели (ru)".to_string(),
            pattern: vec![regex("последние\\s+(2|две)\\s+недели")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "следующие 3 недели (ru)".to_string(),
            pattern: vec![regex("следующие\\s+(3|три)\\s+недели")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "последние 2 месяца (ru)".to_string(),
            pattern: vec![regex("последние\\s+(2|два)\\s+месяца")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "следующие 3 месяца (ru)".to_string(),
            pattern: vec![regex("следующие\\s+(3|три)\\s+месяца")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "последние 2 года (ru)".to_string(),
            pattern: vec![regex("последние\\s+(2|два)\\s+года")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "следующие 3 года (ru)".to_string(),
            pattern: vec![regex("следующие\\s+(3|три)\\s+года")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "13 - 15 июля (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*\\-\\s*(\\d{1,2})\\s+июл[яь]")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None })),
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "с 13 по 15 июля (ru)".to_string(),
            pattern: vec![regex("с\\s*(\\d{1,2})\\s+по\\s*(\\d{1,2})\\s+июл[яь]")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None })),
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "13 июля - 15 июля (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+июл[яь]\\s*\\-\\s*(\\d{1,2})\\s+июл[яь]")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None })),
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "8 авг - 12 авг (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+авг\\.?\\s*\\-\\s*(\\d{1,2})\\s+авг\\.?")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 8, day: day1, year: None })),
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 8, day: day2, year: None })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "в течение 2 недель (ru)".to_string(),
            pattern: vec![regex("в\\s+течение\\s+2\\s+недель")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::RelativeGrain {
                    n: 2,
                    grain: Grain::Week,
                })),
                false,
            ))))),
        },
        Rule {
            name: "до конца дня (ru)".to_string(),
            pattern: vec![regex("до\\s+кон(ец|ца)\\s+дня")],
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
            name: "до конца месяца (ru)".to_string(),
            pattern: vec![regex("до\\s+кон(ец|ца)\\s+месяца")],
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
            name: "после 14ч (ru)".to_string(),
            pattern: vec![regex("после\\s*(\\d{1,2})(\\s*ч|\\s*час(а|ов)?)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
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
            name: "18:30ч - 19:00ч (ru)".to_string(),
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
            name: "17ч10 (ru)".to_string(),
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
        Rule {
            name: "18 февраля (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})(-го|-е)?\\s+феврал[яь]")],
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
            name: "восемнадцатое февраля (ru)".to_string(),
            pattern: vec![regex("восемнадцат(ое|ого)\\s+феврал[яь]")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day: 18,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "1 марта (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+март[а]?")],
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
            name: "первое марта (ru)".to_string(),
            pattern: vec![regex("перв(ое|ого)\\s+март[а]?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "1-е мар. (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})-е\\s+мар\\.?")],
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
            name: "3-его марта 2015 (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})-(его|го)\\s+март[а]?\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (d, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let year: i32 = y.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "3 мар. 2015 (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+мар\\.?\\s+(\\d{4})")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "третьего марта 2015 (ru)".to_string(),
            pattern: vec![regex("третьего\\s+март[а]?\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 3, year: Some(year) })))
            }),
        },
        Rule {
            name: "третье марта 2015 (ru)".to_string(),
            pattern: vec![regex("третье\\s+март[а]?\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 3, year: Some(year) })))
            }),
        },
        Rule {
            name: "к третьему марта 2015 (ru)".to_string(),
            pattern: vec![regex("к\\s+третьему\\s+март[а]?\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 3, year: Some(year) })))
            }),
        },
        Rule {
            name: "15.2 (ru)".to_string(),
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
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "15 Фев (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Фф]ев\\.?")],
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
            name: "15-го Фев (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})-го\\s+[Фф]ев\\.?")],
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
            name: "пятнадцатое февраля (ru)".to_string(),
            pattern: vec![regex("пятнадцат(ое|ого)\\s+феврал[яь]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 15, year: None })))),
        },
        Rule {
            name: "пятнадцатого фев (ru)".to_string(),
            pattern: vec![regex("пятнадцатого\\s+фев\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 15, year: None })))),
        },
        Rule {
            name: "8 августа (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+август[а]?")],
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
            name: "8 Авг (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Аа]вг\\.?")],
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
            name: "восьмое августа (ru)".to_string(),
            pattern: vec![regex("восьм(ое|ого)\\s+август[а]?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 8, day: 8, year: None })))),
        },
        Rule {
            name: "март (ru)".to_string(),
            pattern: vec![regex("март")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(3))))),
        },
        Rule {
            name: "Октябрь 2014 (ru)".to_string(),
            pattern: vec![regex("[Оо]ктябрь\\s*(\\d{4})")],
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
            name: "Ноябрь 2014 (ru)".to_string(),
            pattern: vec![regex("[Нн]оябрь\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
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
            name: "тридцать первое октября 1974 (ru)".to_string(),
            pattern: vec![regex("тридцать первое октября\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 31, year: Some(year) })))
            }),
        },
        Rule {
            name: "31-ое октября 1974 (ru)".to_string(),
            pattern: vec![regex("31-ое октября\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 31, year: Some(year) })))
            }),
        },
        Rule {
            name: "14 апреля 2015 (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+апреля\\s+(\\d{4})")],
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
            name: "Пт, 18 июля 2014 (ru)".to_string(),
            pattern: vec![regex("[Пп]т,?\\s*(\\d{1,2})\\s+июля\\s+(\\d{4})")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 7,
                    day,
                    year: Some(year),
                })))
            }),
        },
        Rule {
            name: "четырнадцатое апреля 2015 (ru)".to_string(),
            pattern: vec![regex("четырнадцат(ое|ого)\\s+апреля\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day: 14, year: Some(year) })))
            }),
        },
        Rule {
            name: "двадцать четвертое фев (ru)".to_string(),
            pattern: vec![regex("двадцать\\s+четверт(ое|ого)\\s+фев\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 24, year: None })))),
        },
        Rule {
            name: "двадцать четвёртое февраля (ru)".to_string(),
            pattern: vec![regex("двадцать\\s+четв(е|ё)рт(ое|ого)\\s+феврал(я|ь)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 24, year: None })))),
        },
        Rule {
            name: "24-ого февраля (ru)".to_string(),
            pattern: vec![regex("24-ого\\s+феврал(я|ь)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 24, year: None })))),
        },
        Rule {
            name: "к двадцать четвёртому февраля (ru)".to_string(),
            pattern: vec![regex("к\\s+двадцать\\s+четв(е|ё)ртому\\s+феврал(я|ь)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 24, year: None })))),
        },
        Rule {
            name: "тридцать первое мая 2015 (ru)".to_string(),
            pattern: vec![regex("тридцать\\s+перв(ое|ого)\\s+мая\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day: 31, year: Some(year) })))
            }),
        },
    ]);
    rules
}
