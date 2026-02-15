use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use crate::dimensions::time_grain::Grain;
use super::{PartOfDay, TimeData, TimeForm};

fn parse_zh_num(s: &str) -> Option<u32> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if let Ok(v) = s.parse::<u32>() {
        return Some(v);
    }
    fn d(c: char) -> Option<u32> {
        match c {
            '零' | '〇' => Some(0),
            '一' => Some(1),
            '二' | '兩' | '两' => Some(2),
            '三' => Some(3),
            '四' => Some(4),
            '五' => Some(5),
            '六' => Some(6),
            '七' => Some(7),
            '八' => Some(8),
            '九' => Some(9),
            _ => None,
        }
    }
    if s == "十" {
        return Some(10);
    }
    if s == "廿" {
        return Some(20);
    }
    if s == "卅" {
        return Some(30);
    }
    if let Some(rest) = s.strip_prefix('廿') {
        let tail = if rest.is_empty() { 0 } else { d(rest.chars().next()?)? };
        return Some(20 + tail);
    }
    if let Some(rest) = s.strip_prefix('卅') {
        let tail = if rest.is_empty() { 0 } else { d(rest.chars().next()?)? };
        return Some(30 + tail);
    }
    if let Some(rest) = s.strip_prefix('十') {
        return Some(10 + d(rest.chars().next()?)?);
    }
    if let Some(rest) = s.strip_suffix('十') {
        return Some(d(rest.chars().next()?)? * 10);
    }
    if let Some((a, b)) = s.split_once('十') {
        let aa = if a.is_empty() { 1 } else { d(a.chars().next()?)? };
        let bb = if b.is_empty() { 0 } else { d(b.chars().next()?)? };
        return Some(aa * 10 + bb);
    }
    d(s.chars().next()?)
}

fn parse_zh_grain(s: &str) -> Option<Grain> {
    if s.starts_with("秒") {
        return Some(Grain::Second);
    }
    if s.starts_with("分鐘") || s.starts_with("分钟") || s.starts_with("分") {
        return Some(Grain::Minute);
    }
    if s.starts_with("小時") || s.starts_with("小时") || s.starts_with("個鐘") || s.starts_with("鐘") || s.starts_with("钟") {
        return Some(Grain::Hour);
    }
    if s == "天" || s == "日" {
        return Some(Grain::Day);
    }
    if s == "周" || s == "週" || s == "礼拜" || s == "禮拜" || s == "星期" {
        return Some(Grain::Week);
    }
    if s == "月" {
        return Some(Grain::Month);
    }
    if s == "年" {
        return Some(Grain::Year);
    }
    None
}

fn fixed_holiday_rule(name: &str, pattern: &str, month: u32, day: u32) -> Rule {
    Rule {
        name: name.to_string(),
        pattern: vec![regex(pattern)],
        production: Box::new(move |_| {
            Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month,
                day,
                year: None,
            })))
        }),
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule { name: "now (zh)".to_string(), pattern: vec![regex("现在|現在|此时|此時|当前|當前|宜家|而家|依家")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))) },
        Rule { name: "today (zh)".to_string(), pattern: vec![regex("今天|今日|此刻")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))) },
        Rule { name: "tomorrow (zh)".to_string(), pattern: vec![regex("明天|明日|聽日")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))) },
        Rule { name: "day after tomorrow (zh)".to_string(), pattern: vec![regex("后天|後天|後日")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))) },
        Rule { name: "day before yesterday (zh)".to_string(), pattern: vec![regex("前天|前日")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))) },
        Rule { name: "morning (zh)".to_string(), pattern: vec![regex("早上|早晨|朝早")], production: Box::new(|_| Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(PartOfDay::Morning))))) },
        Rule { name: "afternoon (zh)".to_string(), pattern: vec![regex("下午|午后|午後")], production: Box::new(|_| Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(PartOfDay::Afternoon))))) },
        Rule { name: "evening (zh)".to_string(), pattern: vec![regex("晚上|晚间|晚間")], production: Box::new(|_| Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(PartOfDay::Evening))))) },
        Rule { name: "night (zh)".to_string(), pattern: vec![regex("夜晚|夜里|夜裡|夜间|夜間")], production: Box::new(|_| Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(PartOfDay::Night))))) },
        Rule { name: "weekday monday (zh)".to_string(), pattern: vec![regex("星期一|週一|周一|礼拜一|禮拜一")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(0))))) },
        Rule { name: "weekday tuesday (zh)".to_string(), pattern: vec![regex("星期二|週二|周二|礼拜二|禮拜二")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(1))))) },
        Rule { name: "weekday wednesday (zh)".to_string(), pattern: vec![regex("星期三|週三|周三|礼拜三|禮拜三")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(2))))) },
        Rule { name: "weekday thursday (zh)".to_string(), pattern: vec![regex("星期四|週四|周四|礼拜四|禮拜四")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(3))))) },
        Rule { name: "weekday friday (zh)".to_string(), pattern: vec![regex("星期五|週五|周五|礼拜五|禮拜五")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(4))))) },
        Rule { name: "weekday saturday (zh)".to_string(), pattern: vec![regex("星期六|週六|周六|礼拜六|禮拜六")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(5))))) },
        Rule { name: "weekday sunday (zh)".to_string(), pattern: vec![regex("星期日|週日|周日|星期天|週天|周天|礼拜天|禮拜天")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(6))))) },
        Rule { name: "this weekend (zh)".to_string(), pattern: vec![regex("这周末|這週末")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))) },
        Rule { name: "this week (zh)".to_string(), pattern: vec![regex("这周|這週|这一周|這一周|這一週|本周|本週")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))) },
        Rule { name: "this week alt (zh)".to_string(), pattern: vec![regex("这礼拜|這禮拜|这一礼拜|這一禮拜|这个礼拜|這個禮拜")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))) },
        Rule { name: "this week cantonese (zh)".to_string(), pattern: vec![regex("今個星期|今个星期|今個禮拜|今個礼拜|今个礼拜")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))) },
        Rule { name: "last week (zh)".to_string(), pattern: vec![regex("上周|上週|上星期|上礼拜|上禮拜|上一周|上一週")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))) },
        Rule { name: "last week cantonese (zh)".to_string(), pattern: vec![regex("上個星期|上个星期|上個禮拜|上個礼拜|上个礼拜")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))) },
        Rule { name: "next week (zh)".to_string(), pattern: vec![regex("下周|下週|下星期|下礼拜|下禮拜|下一周|下一週")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))) },
        Rule { name: "next week cantonese (zh)".to_string(), pattern: vec![regex("下個星期|下个星期|下個禮拜|下個礼拜|下个礼拜")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))) },
        Rule { name: "this month (zh)".to_string(), pattern: vec![regex("本月|这个月|這個月|今月")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Month))))) },
        Rule { name: "last month (zh)".to_string(), pattern: vec![regex("上月|上个月|上個月")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))) },
        Rule { name: "next month (zh)".to_string(), pattern: vec![regex("下月|下个月|下個月")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))) },
        Rule { name: "last year (zh)".to_string(), pattern: vec![regex("去年|上年")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))) },
        Rule { name: "this year (zh)".to_string(), pattern: vec![regex("今年|这一年|這一年|这年|這年|今個年|今个年")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))) },
        Rule { name: "next year (zh)".to_string(), pattern: vec![regex("明年|下年")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))) },
        Rule {
            name: "last n cycle (zh)".to_string(),
            pattern: vec![regex("(上|前)([一二三四五六七八九十兩两廿卅\\d]{1,4})(?:个|個)?(秒(钟|鐘)?|分鐘?|分钟|小時|小时|個鐘|鐘頭?|钟头?|天|日|周|週|礼拜|禮拜|星期|月|年)")],
            production: Box::new(|nodes| {
                let (n, gtxt) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let n = parse_zh_num(n)? as i64;
                let grain = parse_zh_grain(gtxt)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "n cycle last (zh)".to_string(),
            pattern: vec![regex("([一二三四五六七八九十兩两廿卅\\d]{1,4})(?:个|個)?(秒(钟|鐘)?|分鐘?|分钟|小時|小时|個鐘|鐘頭?|钟头?|天|日|周|週|礼拜|禮拜|星期|月|年)(之)?前")],
            production: Box::new(|nodes| {
                let (n, gtxt) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let n = parse_zh_num(n)? as i64;
                let grain = parse_zh_grain(gtxt)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "next n cycle (zh)".to_string(),
            pattern: vec![regex("(下|后|後)([一二三四五六七八九十兩两廿卅\\d]{1,4})(?:个|個)?(秒(钟|鐘)?|分鐘?|分钟|小時|小时|個鐘|鐘頭?|钟头?|天|日|周|週|礼拜|禮拜|星期|月|年)")],
            production: Box::new(|nodes| {
                let (n, gtxt) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let n = parse_zh_num(n)? as i64;
                let grain = parse_zh_grain(gtxt)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "n cycle next (zh)".to_string(),
            pattern: vec![regex("([一二三四五六七八九十兩两廿卅\\d]{1,4})(?:个|個)?(秒(钟|鐘)?|分鐘?|分钟|小時|小时|個鐘|鐘頭?|钟头?|天|日|周|週|礼拜|禮拜|星期|月|年)(之)?(后|後)")],
            production: Box::new(|nodes| {
                let (n, gtxt) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let n = parse_zh_num(n)? as i64;
                let grain = parse_zh_grain(gtxt)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule { name: "last sunday (zh)".to_string(), pattern: vec![regex("上禮拜日|上礼拜日|上週日|上周日")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(6))))) },
        Rule {
            name: "month-day chinese numerals (zh)".to_string(),
            pattern: vec![regex("([一二三四五六七八九十兩两廿卅\\d]{1,3})月([一二三四五六七八九十兩两廿卅\\d]{1,3})(日|号|號)")],
            production: Box::new(|nodes| {
                let (m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let month = parse_zh_num(m)?;
                let day = parse_zh_num(d)?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "month only (zh)".to_string(),
            pattern: vec![regex("([一二三四五六七八九十兩两廿卅\\d]{1,3})月")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month = parse_zh_num(m)?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "weekday comma month-day (zh)".to_string(),
            pattern: vec![regex("(星期|週|周|礼拜|禮拜)[一二三四五六日天],?\\s*([一二三四五六七八九十兩两廿卅\\d]{1,3})月([一二三四五六七八九十兩两廿卅\\d]{1,3})(日|号|號)")],
            production: Box::new(|nodes| {
                let (m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let month = parse_zh_num(m)?;
                let day = parse_zh_num(d)?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "relative month + day (zh)".to_string(),
            pattern: vec![regex("(本月|这个月|這個月|今月|今個月|今个月|上月|上个月|上個月|下月|下个月|下個月)([一二三四五六七八九十兩两廿卅\\d]{1,3})(日|号|號)")],
            production: Box::new(|nodes| {
                let (prefix, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let day = parse_zh_num(d)?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                let month_base = if matches!(prefix, "上月" | "上个月" | "上個月") {
                    TimeForm::GrainOffset { grain: Grain::Month, offset: -1 }
                } else if matches!(prefix, "下月" | "下个月" | "下個月") {
                    TimeForm::GrainOffset { grain: Grain::Month, offset: 1 }
                } else {
                    TimeForm::AllGrain(Grain::Month)
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(month_base)),
                    Box::new(TimeData::new(TimeForm::DayOfMonth(day))),
                ))))
            }),
        },
        Rule { name: "yesterday (zh)".to_string(), pattern: vec![regex("昨天|昨日|尋日")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))) },
        Rule {
            name: "hour-of-day (zh)".to_string(),
            pattern: vec![regex("([零〇一二三四五六七八九十兩两廿卅\\d]{1,3})(点|點|時)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hour = parse_zh_num(h)?;
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Hour(hour, true))))
            }),
        },
        Rule {
            name: "hh:mm (zh)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))[:：]([0-5]\\d)")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hour = h.parse::<u32>().ok()?;
                let minute = m.parse::<u32>().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, true))))
            }),
        },
        Rule {
            name: "guoqing (zh)".to_string(),
            pattern: vec![regex("国庆|國慶")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 1, year: None })))),
        },
        Rule {
            name: "new year day (zh)".to_string(),
            pattern: vec![regex("元旦(节|節)?|((公|(阳|陽))(历|曆))?新年")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("new year".to_string(), None))))),
        },
        fixed_holiday_rule("women day (zh)", "(国际劳动|國際勞動|三八)?(妇|婦)女(节|節)", 3, 8),
        fixed_holiday_rule("labor day (zh)", "(五一|51)?(国际|國際)?(劳动|勞動)(节|節)", 5, 1),
        fixed_holiday_rule("youth day (zh)", "(中(国|國))?(五四|54)?青年(节|節)", 5, 4),
        fixed_holiday_rule("children day (zh)", "(国际|國際)?(六一|61)?(儿|兒)童(节|節)", 6, 1),
        fixed_holiday_rule("teachers day (zh)", "(中(国|國))?教师(节|節)", 9, 10),
        fixed_holiday_rule("halloween (zh)", "万圣节|萬聖節", 10, 31),
        fixed_holiday_rule("singles day (zh)", "光棍(节|節)|(双|雙)(十一|11)", 11, 11),
        fixed_holiday_rule("christmas eve (zh)", "(平安|聖誕)夜", 12, 24),
        fixed_holiday_rule("christmas day (zh)", "(圣诞|聖誕)(节|節)?", 12, 25),
        fixed_holiday_rule("new year eve (zh)", "新年夜", 12, 31),
        fixed_holiday_rule("valentine day (zh)", "(情人|(圣瓦伦丁|聖瓦倫丁))(节|節)", 2, 14),
        fixed_holiday_rule("white day (zh)", "白色情人(节|節)", 3, 14),
        fixed_holiday_rule("fools day (zh)", "愚人(节|節)", 4, 1),
        fixed_holiday_rule("qingming (zh)", "清明(节|節)", 4, 5),
        fixed_holiday_rule("consumer rights day (zh)", "(国际|世界)?(消费者权益|消費者權益)日|三一五", 3, 15),
        fixed_holiday_rule("world environment day (zh)", "世界(环|環)境日", 6, 5),
        fixed_holiday_rule("world ocean day (zh)", "世界海洋日", 6, 8),
        fixed_holiday_rule("world blood donor day (zh)", "世界(献|獻)血日", 6, 14),
        fixed_holiday_rule("world refugee day (zh)", "世界(难|難)民日", 6, 20),
        fixed_holiday_rule("world yoga day (zh)", "(国际|國際)瑜伽日", 6, 21),
        fixed_holiday_rule("world olympic day (zh)", "(国际|國際)奥林匹克日", 6, 23),
        fixed_holiday_rule("us independence day (zh)", "(美国)?(独|獨)立日", 7, 4),
        fixed_holiday_rule("army day (zh)", "(中国人民解放(军|軍)|八一)?建(军节|軍節)", 8, 1),
        fixed_holiday_rule("intl youth day (zh)", "(国际|國際)青年(节|節)", 8, 12),
        fixed_holiday_rule("xinhai memorial (zh)", "辛亥革命(纪|紀)念日", 10, 10),
        fixed_holiday_rule("world food day (zh)", "世界((粮|糧)食|食物)日", 10, 16),
        fixed_holiday_rule("veterans day (zh)", "(退伍(军|軍)人|老兵)(节|節)", 11, 11),
        fixed_holiday_rule("world diabetes day (zh)", "世界糖尿病日", 11, 14),
        fixed_holiday_rule("human rights day (zh)", "人(权|權)日", 12, 10),
        fixed_holiday_rule("nanjing memorial day (zh)", "南京大屠(杀纪|殺紀)念日", 12, 13),
        fixed_holiday_rule("macao handover day (zh)", "澳(门|門)回(归纪|歸紀)念日", 12, 20),
        Rule {
            name: "chinese new year (zh)".to_string(),
            pattern: vec![regex("春(节|節)|(农历|農曆|唐人)新年|新(正|春)|正月(正(时|時)|朔日)|岁首")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("chinese new year".to_string(), None))))),
        },
        Rule {
            name: "easter (zh)".to_string(),
            pattern: vec![regex("(复|復)活(节|節)|主(复|復)活日")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("easter".to_string(), None))))),
        },
        Rule {
            name: "easter monday (zh)".to_string(),
            pattern: vec![regex("(复|復)活(节|節)星期一")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("easter monday".to_string(), None))))),
        },
        Rule {
            name: "good friday (zh)".to_string(),
            pattern: vec![regex("(耶(稣|穌)|主)受(难|難)(节|節|日)|(圣|聖|沈默)(周|週)五")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("good friday".to_string(), None))))),
        },
        Rule {
            name: "ash wednesday (zh)".to_string(),
            pattern: vec![regex("大(斋|齋)首日|(圣|聖)灰((礼仪|禮儀)?日|星期三)|灰日")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("ash wednesday".to_string(), None))))),
        },
        Rule {
            name: "pentecost (zh)".to_string(),
            pattern: vec![regex("五旬(节|節)|(圣灵|聖靈)降(临|臨)(日|节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("pentecost".to_string(), None))))),
        },
        Rule {
            name: "trinity sunday (zh)".to_string(),
            pattern: vec![regex("((天主)?(圣|聖)?三一|(圣|聖)三)(主日|节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("trinity sunday".to_string(), None))))),
        },
        Rule {
            name: "palm sunday (zh)".to_string(),
            pattern: vec![regex("((棕|圣|聖)枝|圣树|聖樹|基督苦(难|難))主日")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("palm sunday".to_string(), None))))),
        },
        Rule {
            name: "yom kippur (zh)".to_string(),
            pattern: vec![regex("(赎|贖)罪日")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("yom kippur".to_string(), None))))),
        },
        Rule {
            name: "eid al-fitr (zh)".to_string(),
            pattern: vec![regex("(开斋|開齋|肉孜|(尔|爾)代)(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("eid al-fitr".to_string(), None))))),
        },
        Rule {
            name: "eid al-adha (zh)".to_string(),
            pattern: vec![regex("古(尔|爾)邦(节|節)|宰牲(节|節)|难近母(节|節)|難近母(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("eid al-adha".to_string(), None))))),
        },
        Rule {
            name: "mawlid (zh)".to_string(),
            pattern: vec![regex("圣纪节|聖紀節")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("mawlid".to_string(), None))))),
        },
        Rule {
            name: "islamic new year (zh)".to_string(),
            pattern: vec![regex("伊斯兰(教)?(历)?新年")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("islamic new year".to_string(), None))))),
        },
        Rule {
            name: "ashura (zh)".to_string(),
            pattern: vec![regex("阿舒拉(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("ashura".to_string(), None))))),
        },
        Rule {
            name: "chhath (zh)".to_string(),
            pattern: vec![regex("克哈特普迦(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("chhath".to_string(), None))))),
        },
        Rule {
            name: "diwali (zh)".to_string(),
            pattern: vec![regex("(排|万|萬|印度)(灯节|燈節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("diwali".to_string(), None))))),
        },
        Rule {
            name: "holi (zh)".to_string(),
            pattern: vec![regex("((侯|荷)(丽|麗)|洒红|灑紅|欢悦|歡悅|五彩|胡里|好利|霍利)(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("holi".to_string(), None))))),
        },
        Rule {
            name: "dussehra (zh)".to_string(),
            pattern: vec![regex("(十(胜|勝)|(凯|凱)旋|(圣|聖)母)(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("dussehra".to_string(), None))))),
        },
        Rule {
            name: "onam (zh)".to_string(),
            pattern: vec![regex("欧南(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("onam".to_string(), None))))),
        },
        Rule {
            name: "raksha bandhan (zh)".to_string(),
            pattern: vec![regex("(印度兄妹|拉克沙班丹)(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("raksha bandhan".to_string(), None))))),
        },
        Rule {
            name: "vesak (zh)".to_string(),
            pattern: vec![regex("((卫|衛)塞|威瑟|比(萨宝|薩寶)蕉)(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("vesak".to_string(), None))))),
        },
        Rule {
            name: "tisha b'av (zh)".to_string(),
            pattern: vec![regex("((圣|聖)殿被毁|禁食)日")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("tisha b'av".to_string(), None))))),
        },
        Rule {
            name: "tu bishvat (zh)".to_string(),
            pattern: vec![regex("((犹|猶)太植(树|樹)|(图|圖)比舍巴特)(节|節)|(树|樹)木新年")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("tu bishvat".to_string(), None))))),
        },
        Rule {
            name: "ascension day (zh)".to_string(),
            pattern: vec![regex("耶(稣|穌)升天(节|節|日)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("ascension day".to_string(), None))))),
        },
        Rule {
            name: "corpus christi (zh)".to_string(),
            pattern: vec![regex("基督(圣体|聖體)((圣|聖)血)?((节|節)|瞻(礼|禮))")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("corpus christi".to_string(), None))))),
        },
        Rule {
            name: "holy saturday (zh)".to_string(),
            pattern: vec![regex("神?(圣周|聖週)六|(耶(稣|穌)|主)受(难|難)(节|節|日)翌日|(复|復)活(节|節)前夜|黑色星期六")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("holy saturday".to_string(), None))))),
        },
        Rule {
            name: "orthodox easter monday (zh)".to_string(),
            pattern: vec![regex("(东|東)正教(复|復)活(节|節)星期一")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("orthodox easter monday".to_string(), None))))),
        },
        Rule {
            name: "orthodox easter (zh)".to_string(),
            pattern: vec![regex("(东|東)正教((复|復)活(节|節)|主(复|復)活日)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("orthodox easter".to_string(), None))))),
        },
        Rule {
            name: "orthodox holy saturday (zh)".to_string(),
            pattern: vec![regex("(东|東)正教(神?(圣周|聖週)六|(耶(稣|穌)|主)受(难|難)(节|節|日)翌日|(复|復)活(节|節)前夜)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("orthodox holy saturday".to_string(), None))))),
        },
        Rule {
            name: "orthodox good friday (zh)".to_string(),
            pattern: vec![regex("(东|東)正教((耶(稣|穌)|主)受(难|難)(节|節|日)|(圣|聖|沈默)(周|週)五)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("orthodox good friday".to_string(), None))))),
        },
        Rule {
            name: "ascension to heaven (zh)".to_string(),
            pattern: vec![regex("(夜行)?登霄(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("ascension to heaven".to_string(), None))))),
        },
        Rule {
            name: "thai pongal (zh)".to_string(),
            pattern: vec![regex("(印度|淡米(尔|爾))(丰|豐)收(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("thai pongal".to_string(), None))))),
        },
        Rule {
            name: "maattu pongal (zh)".to_string(),
            pattern: vec![regex("(印度(丰|豐)收|(庞|龐)格(尔|爾))(节|節)第三天")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("maattu pongal".to_string(), None))))),
        },
        Rule {
            name: "kaanum pongal (zh)".to_string(),
            pattern: vec![regex("(印度(丰|豐)收|(庞|龐)格(尔|爾))(节|節)第四天")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("kaanum pongal".to_string(), None))))),
        },
        Rule {
            name: "lag baomer (zh)".to_string(),
            pattern: vec![regex("((犹|猶)太教)?篝火(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("lag baomer".to_string(), None))))),
        },
        Rule {
            name: "night of decree (zh)".to_string(),
            pattern: vec![regex("(法令|命运|权力)之夜")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("night of decree".to_string(), None))))),
        },
        Rule {
            name: "maundy thursday (zh)".to_string(),
            pattern: vec![regex("濯足(节|節)|神(圣|聖)星期四|(圣周|聖週)(星期)?四|(设|設)立(圣|聖)餐日")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("maundy thursday".to_string(), None))))),
        },
        Rule {
            name: "shemini atzeret (zh)".to_string(),
            pattern: vec![regex("(圣|聖)会(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("shemini atzeret".to_string(), None))))),
        },
        Rule {
            name: "simchat torah (zh)".to_string(),
            pattern: vec![regex("(西赫(托拉|妥拉)|诵经|誦經|转经|轉經|律法|(欢庆圣|歡慶聖)法)(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("simchat torah".to_string(), None))))),
        },
        Rule {
            name: "mardi gras (zh)".to_string(),
            pattern: vec![regex("忏悔(节|節|火曜日)|煎(饼|餅)星期二")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("mardi gras".to_string(), None))))),
        },
        Rule {
            name: "monday of holy spirit (zh)".to_string(),
            pattern: vec![regex("(圣灵节庆|聖靈節慶)日")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("monday of the holy spirit".to_string(), None))))),
        },
        Rule {
            name: "boss day (zh)".to_string(),
            pattern: vec![regex("老(板节|闆節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("boss day".to_string(), None))))),
        },
        Rule {
            name: "global youth service day (zh)".to_string(),
            pattern: vec![regex("全球青年服(务|務)日")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("global youth service day".to_string(), None))))),
        },
        Rule {
            name: "great fast (zh)".to_string(),
            pattern: vec![regex("四旬(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("great fast".to_string(), None))))),
        },
        Rule {
            name: "chanukah (zh)".to_string(),
            pattern: vec![regex("(光明|修殿|(献|獻)殿|(烛|燭)光|哈努卡|(马|馬)加比)(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("chanukah".to_string(), None))))),
        },
        Rule {
            name: "lent (zh)".to_string(),
            pattern: vec![regex("大(斋|齋)(期|节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("lent".to_string(), None))))),
        },
        Rule {
            name: "navaratri (zh)".to_string(),
            pattern: vec![regex("(九夜|那瓦拉提)(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("navaratri".to_string(), None))))),
        },
        Rule {
            name: "passover (zh)".to_string(),
            pattern: vec![regex("逾越(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("passover".to_string(), None))))),
        },
        Rule {
            name: "ramadan (zh)".to_string(),
            pattern: vec![regex("斋月|齋月|斋戒月|齋戒月|莱麦丹|萊麥丹|拉马丹|拉馬丹")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("ramadan".to_string(), None))))),
        },
        Rule {
            name: "rosh hashanah (zh)".to_string(),
            pattern: vec![regex("(犹|猶)太新年|吹角(节|節)|(犹太)?新年(节|節)|(犹|猶)太历新年")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("rosh hashanah".to_string(), None))))),
        },
        Rule {
            name: "shavuot (zh)".to_string(),
            pattern: vec![regex("(五旬|七七|收获|收穫)(节|節)|圣灵降临节|聖靈降臨節|沙夫幼特(节|節)|新果实(节|節)|新果實(节|節)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("shavuot".to_string(), None))))),
        },
        Rule {
            name: "sukkot (zh)".to_string(),
            pattern: vec![regex("(住棚|帐棚|帳棚|结茅|結茅)(节|節)|住棚(节|節)?|苏科特")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("sukkot".to_string(), None))))),
        },
        Rule {
            name: "earth hour (zh)".to_string(),
            pattern: vec![regex("地球一小(时|時)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("earth hour".to_string(), None))))),
        },
    ]);
    rules
}
