use crate::dimensions::time_grain::Grain;
use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{Direction, IntervalDirection, PartOfDay, TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ko)".to_string(),
            pattern: vec![regex("방금|지금")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ko)".to_string(),
            pattern: vec![regex("오늘")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ko)".to_string(),
            pattern: vec![regex("내일")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ko)".to_string(),
            pattern: vec![regex("어제")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (ko)".to_string(),
            pattern: vec![regex("월(요일|욜)|화(요일|욜)|수(요일|욜)|목(요일|욜)|금(요일|욜)|토(요일|욜)|일(요일|욜)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_string(),
                    _ => return None,
                };
                let dow = if s.starts_with('월') {
                    0
                } else if s.starts_with('화') {
                    1
                } else if s.starts_with('수') {
                    2
                } else if s.starts_with('목') {
                    3
                } else if s.starts_with('금') {
                    4
                } else if s.starts_with('토') {
                    5
                } else if s.starts_with('일') {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "크리스마스 (ko)".to_string(),
            pattern: vec![regex("크리스마스|성탄절")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 12,
                day: 25,
                year: None,
            })))),
        },
        Rule {
            name: "신정 (ko)".to_string(),
            pattern: vec![regex("신정")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 1,
                day: 1,
                year: None,
            })))),
        },
        Rule {
            name: "설날 (ko)".to_string(),
            pattern: vec![regex("설날")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "chinese new year".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "삼일절 (ko)".to_string(),
            pattern: vec![regex("삼일절")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 3,
                day: 1,
                year: None,
            })))),
        },
        Rule {
            name: "어린이날 (ko)".to_string(),
            pattern: vec![regex("어린이날")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 5,
                day: 5,
                year: None,
            })))),
        },
        Rule {
            name: "현충일 (ko)".to_string(),
            pattern: vec![regex("현충일")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 6,
                day: 6,
                year: None,
            })))),
        },
        Rule {
            name: "제헌절 (ko)".to_string(),
            pattern: vec![regex("제헌절")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 7,
                day: 17,
                year: None,
            })))),
        },
        Rule {
            name: "광복절 (ko)".to_string(),
            pattern: vec![regex("광복절")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 8,
                day: 15,
                year: None,
            })))),
        },
        Rule {
            name: "개천절 (ko)".to_string(),
            pattern: vec![regex("개천절")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 10,
                day: 3,
                year: None,
            })))),
        },
        Rule {
            name: "한글날 (ko)".to_string(),
            pattern: vec![regex("한글날")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 10,
                day: 9,
                year: None,
            })))),
        },
        Rule {
            name: "크리스마스 이브 (ko)".to_string(),
            pattern: vec![regex("(크리스마스)?이브")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 12,
                day: 24,
                year: None,
            })))),
        },
        Rule {
            name: "month/day (ko)".to_string(),
            pattern: vec![regex("(\\d{1,2})월\\s*(\\d{1,2})일")],
            production: Box::new(|nodes| {
                let (m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
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
            name: "3월 (ko)".to_string(),
            pattern: vec![regex("(\\d{1,2})월")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "year/month (ko)".to_string(),
            pattern: vec![regex("(\\d{4})년\\s*(\\d{1,2})월")],
            production: Box::new(|nodes| {
                let (y, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day: 1,
                    year: Some(year),
                })))
            }),
        },
        Rule {
            name: "full spoken ymd (ko)".to_string(),
            pattern: vec![regex("이천십오년\\s*삼월\\s*삼일")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 3,
                    year: Some(2015),
                })))
            }),
        },
        Rule {
            name: "dom with suffix (ko)".to_string(),
            pattern: vec![regex("(\\d{1,2})일에")],
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
            name: "yy/mm/dd (ko)".to_string(),
            pattern: vec![regex("(\\d{2})/(\\d{1,2})/(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (yy, m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let year2: i32 = yy.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: Some(1900 + year2),
                })))
            }),
        },
        Rule {
            name: "next month name (ko)".to_string(),
            pattern: vec![regex("다음\\s*(\\d{1,2})월")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::Month(month));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "this week (ko)".to_string(),
            pattern: vec![regex("이번(주)?|금주|다음주|오는주")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "last week (ko)".to_string(),
            pattern: vec![regex("저번(주)?|지난(주)?|전(주)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "next month (ko)".to_string(),
            pattern: vec![regex("다음달|다음\\s*달")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "next quarter (ko)".to_string(),
            pattern: vec![regex("다음분기|다음\\s*분기")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "삼분기 (ko)".to_string(),
            pattern: vec![regex("삼분기|3분기")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "2018년 4분기 (ko)".to_string(),
            pattern: vec![regex("(\\d{4})년\\s*(\\d)분기")],
            production: Box::new(|nodes| {
                let (y, q) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let quarter: u32 = q.parse().ok()?;
                if !(1..=4).contains(&quarter) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(quarter, year))))
            }),
        },
        Rule {
            name: "작년 (ko)".to_string(),
            pattern: vec![regex("작년|지난해")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "12년 (ko)".to_string(),
            pattern: vec![regex("(\\d{2})년")],
            production: Box::new(|nodes| {
                let yy = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year2: i32 = yy.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(2000 + year2))))
            }),
        },
        Rule {
            name: "1954 (ko)".to_string(),
            pattern: vec![regex("(\\d{4})")],
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
            name: "올해 (ko)".to_string(),
            pattern: vec![regex("올해|금년")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 0 })))),
        },
        Rule {
            name: "내년 (ko)".to_string(),
            pattern: vec![regex("내년")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "모레 (ko)".to_string(),
            pattern: vec![regex("모레")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))),
        },
        Rule {
            name: "엊그제 (ko)".to_string(),
            pattern: vec![regex("엊그제|엊그저께")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))),
        },
        Rule {
            name: "그제 (ko)".to_string(),
            pattern: vec![regex("그제")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))),
        },
        Rule {
            name: "아침 3시 (ko)".to_string(),
            pattern: vec![regex("아침\\s*(\\d{1,2})시")],
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
            name: "저녁 8시 (ko)".to_string(),
            pattern: vec![regex("저녁\\s*(\\d{1,2})시|밤\\s*(\\d{1,2})시")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if hour > 12 {
                    return None;
                }
                if hour < 12 {
                    hour += 12;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "1초안에 (ko)".to_string(),
            pattern: vec![regex("(\\d{1,2})초안에")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let secs: i32 = s.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Second, offset: secs })))
            }),
        },
        Rule {
            name: "일분안에 (ko)".to_string(),
            pattern: vec![regex("일분안에|1분안에")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 1 })))),
        },
        Rule {
            name: "일분내에 (ko)".to_string(),
            pattern: vec![regex("일분내에|1분내에")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 1 })))),
        },
        Rule {
            name: "이분안에 (ko)".to_string(),
            pattern: vec![regex("이분안에|2분안에")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 2 })))),
        },
        Rule {
            name: "이분내에 (ko)".to_string(),
            pattern: vec![regex("이분내에|2분내에")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 2 })))),
        },
        Rule {
            name: "15분안/내 (ko)".to_string(),
            pattern: vec![regex("(\\d{1,3})분(안|내)(에)?")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mins: i32 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: mins })))
            }),
        },
        Rule {
            name: "한시간안에 (ko)".to_string(),
            pattern: vec![regex("한시간안에|1시간안에|한시간내(에)?|1시간내(에)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 1 })))),
        },
        Rule {
            name: "24시간안에 (ko)".to_string(),
            pattern: vec![regex("(\\d{1,2})시간(안|내)(에)?")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hours: i32 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: hours })))
            }),
        },
        Rule {
            name: "7일안에 (ko)".to_string(),
            pattern: vec![regex("(\\d{1,2})일(안|내)(에)?")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let days: i32 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: days })))
            }),
        },
        Rule {
            name: "1주일안에 (ko)".to_string(),
            pattern: vec![regex("(\\d{1,2})주일(안|내)(에)?")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let weeks: i32 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: weeks })))
            }),
        },
        Rule {
            name: "2주 이내에 (ko)".to_string(),
            pattern: vec![regex("(\\d{1,2})주\\s*이내(에)?")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let weeks: i64 = n.parse().ok()?;
                let start = TimeData::new(TimeForm::Now);
                let end = TimeData::new(TimeForm::RelativeGrain { n: weeks, grain: Grain::Week });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(start),
                    Box::new(end),
                    false,
                ))))
            }),
        },
        Rule {
            name: "하루안에 (ko)".to_string(),
            pattern: vec![regex("하루(안|내)(에)?|1일(안|내)(에)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Day,
                offset: 1,
            })))),
        },
        Rule {
            name: "삼년안에 (ko)".to_string(),
            pattern: vec![regex("(일|이|삼|사|오|육|칠|팔|구|십)년(안|내)(에)?|(\\d{1,2})년(안|내)(에)?")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => {
                        if let Some(k) = m.group(1) {
                            match k {
                                "일" => 1,
                                "이" => 2,
                                "삼" => 3,
                                "사" => 4,
                                "오" => 5,
                                "육" => 6,
                                "칠" => 7,
                                "팔" => 8,
                                "구" => 9,
                                "십" => 10,
                                _ => return None,
                            }
                        } else {
                            m.group(4)?.parse().ok()?
                        }
                    }
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: n })))
            }),
        },
        Rule {
            name: "다음 3분/시간/일/주/달/년 (ko)".to_string(),
            pattern: vec![regex("다음\\s*(\\d{1,3})\\s*(초|분|시간|일|주|주일|달|개월|년)")],
            production: Box::new(|nodes| {
                let (n, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let val: i32 = n.parse().ok()?;
                let grain = if u == "초" {
                    Grain::Second
                } else if u == "분" {
                    Grain::Minute
                } else if u == "시간" {
                    Grain::Hour
                } else if u == "일" {
                    Grain::Day
                } else if u == "주" || u == "주일" {
                    Grain::Week
                } else if u == "달" || u == "개월" {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: val })))
            }),
        },
        Rule {
            name: "다음 삼분/이시간... (ko)".to_string(),
            pattern: vec![regex("다음\\s*(일|이|삼|사|오|육|칠|팔|구|십|한|두|세|네|다섯|여섯|일곱|여덟|아홉|열)\\s*(초|분|시간|일|주|주일|달|개월|년)")],
            production: Box::new(|nodes| {
                let (n, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let val: i32 = match n {
                    "일" | "한" => 1,
                    "이" | "두" => 2,
                    "삼" | "세" => 3,
                    "사" | "네" => 4,
                    "오" | "다섯" => 5,
                    "육" | "여섯" => 6,
                    "칠" | "일곱" => 7,
                    "팔" | "여덟" => 8,
                    "구" | "아홉" => 9,
                    "십" | "열" => 10,
                    _ => return None,
                };
                let grain = if u == "초" {
                    Grain::Second
                } else if u == "분" {
                    Grain::Minute
                } else if u == "시간" {
                    Grain::Hour
                } else if u == "일" {
                    Grain::Day
                } else if u == "주" || u == "주일" {
                    Grain::Week
                } else if u == "달" || u == "개월" {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: val })))
            }),
        },
        Rule {
            name: "한시간반안 (ko)".to_string(),
            pattern: vec![regex("약\\s*한시간반\\s*안에|한시간반안(에)?|1시간반안(에)?|한시간반내(에)?|1시간반내(에)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 90 })))),
        },
        Rule {
            name: "두시간반안 (ko)".to_string(),
            pattern: vec![regex("두시간반안(에)?|2시간반안(에)?|두시간반내(에)?|2시간반내(에)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 150 })))),
        },
        Rule {
            name: "몇시간안/내 (ko)".to_string(),
            pattern: vec![regex("몇시간(안|내)(에)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 3 })))),
        },
        Rule {
            name: "몇시간후 (ko)".to_string(),
            pattern: vec![regex("몇시간(후|이후)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 3 })))),
        },
        Rule {
            name: "5일 후 (ko)".to_string(),
            pattern: vec![regex("(\\d{1,3})\\s*(초|분|시간|일|주|달|개월|년)\\s*(후|이후)")],
            production: Box::new(|nodes| {
                let (n, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let val: i32 = n.parse().ok()?;
                let grain = if u == "초" {
                    Grain::Second
                } else if u == "분" {
                    Grain::Minute
                } else if u == "시간" {
                    Grain::Hour
                } else if u == "일" {
                    Grain::Day
                } else if u == "주" {
                    Grain::Week
                } else if u == "달" || u == "개월" {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: val })))
            }),
        },
        Rule {
            name: "오후 세시 (ko)".to_string(),
            pattern: vec![regex("오후\\s*세시")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))),
        },
        Rule {
            name: "오후 2시까지 (ko)".to_string(),
            pattern: vec![regex("(오전|오후)\\s*(\\d{1,2})시까지")],
            production: Box::new(|nodes| {
                let (ap, h) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if hour > 12 || hour == 0 {
                    return None;
                }
                if ap == "오후" && hour < 12 {
                    hour += 12;
                } else if ap == "오전" && hour == 12 {
                    hour = 0;
                }
                let mut t = TimeData::new(TimeForm::HourMinute(hour, 0, false));
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "오후두시에 (ko)".to_string(),
            pattern: vec![regex("(오전|오후)\\s*(한|두|세|네|다섯|여섯|일곱|여덟|아홉|열|\\d{1,2})시(에)?")],
            production: Box::new(|nodes| {
                let (ap, hs) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let mut hour: u32 = match hs {
                    "한" => 1,
                    "두" => 2,
                    "세" => 3,
                    "네" => 4,
                    "다섯" => 5,
                    "여섯" => 6,
                    "일곱" => 7,
                    "여덟" => 8,
                    "아홉" => 9,
                    "열" => 10,
                    _ => hs.parse().ok()?,
                };
                if hour > 12 || hour == 0 {
                    return None;
                }
                if ap == "오후" && hour < 12 {
                    hour += 12;
                } else if ap == "오전" && hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "오후에 (ko)".to_string(),
            pattern: vec![regex("오후에")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Today)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))),
            ))))),
        },
        Rule {
            name: "점심이후 (ko)".to_string(),
            pattern: vec![regex("점심(이)?후")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::HourMinute(12, 0, false));
                t.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "정오 (ko)".to_string(),
            pattern: vec![regex("정오")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))),
        },
        Rule {
            name: "자정 (ko)".to_string(),
            pattern: vec![regex("자정")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(0, 0, false))))),
        },
        Rule {
            name: "세시반 (ko)".to_string(),
            pattern: vec![regex("세시반")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(3, 30, true))))),
        },
        Rule {
            name: "세시이십삼분이십사초 (ko)".to_string(),
            pattern: vec![regex("세시이십삼분이십사초")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinuteSecond(3, 23, 24))))),
        },
    ]);
    rules
}
