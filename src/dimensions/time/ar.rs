use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use crate::dimensions::time_grain::Grain;
use super::{TimeData, TimeForm};

fn ar_ordinal_day(s: &str) -> Option<u32> {
    match s {
        "اول" | "أول" => Some(1),
        "ثاني" => Some(2),
        "ثالث" => Some(3),
        "رابع" => Some(4),
        "خامس" => Some(5),
        "سادس" => Some(6),
        "سابع" => Some(7),
        "ثامن" => Some(8),
        "تاسع" => Some(9),
        "عاشر" => Some(10),
        _ => None,
    }
}

fn ar_ordinal_n(s: &str) -> Option<i32> {
    match s {
        "اول" | "أول" => Some(1),
        "ثان" | "ثاني" => Some(2),
        "ثالث" => Some(3),
        "رابع" => Some(4),
        _ => None,
    }
}

fn ar_month_num(s: &str) -> Option<u32> {
    if let Ok(v) = s.parse::<u32>() {
        return Some(v);
    }
    match s {
        "واحد" | "الأول" | "الاول" => Some(1),
        "اثنين" | "إثنين" | "اثنان" => Some(2),
        "ثلاثة" => Some(3),
        "أربعة" | "اربعة" => Some(4),
        "خمسة" => Some(5),
        "ستة" => Some(6),
        "سبعة" => Some(7),
        "ثمانية" => Some(8),
        "تسعة" => Some(9),
        "عشرة" => Some(10),
        "الثاني" => Some(2),
        "الثالث" => Some(3),
        "الرابع" => Some(4),
        "الخامس" => Some(5),
        "السادس" => Some(6),
        "السابع" => Some(7),
        "الثامن" => Some(8),
        "التاسع" => Some(9),
        "العاشر" => Some(10),
        "الحادي عشر" => Some(11),
        "الثاني عشر" => Some(12),
        "أحد عشر" | "احد عشر" => Some(11),
        "اثنا عشر" | "اثنى عشر" => Some(12),
        _ => None,
    }
}

fn ar_day_of_week(s: &str) -> Option<u32> {
    match s {
        "الاثنين" | "الإثنين" | "اثنين" | "إثنين" => Some(0),
        "الثلاثاء" | "الثلاثا" | "ثلاثاء" | "ثلاثا" => Some(1),
        "الاربعاء" | "الأربعاء" | "اربعاء" | "أربعاء" => Some(2),
        "الخميس" | "خميس" => Some(3),
        "الجمعة" | "جمعه" | "جمعة" => Some(4),
        "السبت" | "سبت" => Some(5),
        "الاحد" | "الأحد" | "احد" | "أحد" => Some(6),
        _ => None,
    }
}

fn ar_hour_ordinal_feminine(s: &str) -> Option<u32> {
    match s {
        "الاولى" | "الأولى" => Some(1),
        "الثانية" => Some(2),
        "الثالثة" => Some(3),
        "الرابعة" => Some(4),
        "الخامسة" => Some(5),
        "السادسة" => Some(6),
        "السابعة" => Some(7),
        "الثامنة" => Some(8),
        "التاسعة" => Some(9),
        "العاشرة" => Some(10),
        "الحادية عشرة" => Some(11),
        "الثانية عشرة" => Some(12),
        _ => None,
    }
}

fn ar_month_name(s: &str) -> Option<u32> {
    match s {
        "يناير" | "كانون الثاني" => Some(1),
        "فبراير" | "شباط" => Some(2),
        "مارس" | "آذار" | "اذار" => Some(3),
        "أبريل" | "ابريل" | "نيسان" => Some(4),
        "مايو" | "أيار" | "ايار" => Some(5),
        "يونيو" | "حزيران" => Some(6),
        "يوليو" | "تموز" => Some(7),
        "أغسطس" | "اغسطس" | "آب" | "اب" => Some(8),
        "سبتمبر" | "أيلول" | "ايلول" => Some(9),
        "أكتوبر" | "اكتوبر" | "تشرين الأول" | "تشرين الاول" => Some(10),
        "نوفمبر" | "تشرين الثاني" => Some(11),
        "ديسمبر" | "كانون الأول" | "كانون الاول" => Some(12),
        _ => None,
    }
}

fn ar_ones_word(s: &str) -> Option<u32> {
    match s {
        "واحد" | "واحدة" | "اول" | "أول" | "الاولى" | "الأولى" => Some(1),
        "اثنين" | "إثنين" | "اثنان" | "اثنتين" | "اثنتان" | "ثان" | "ثاني" | "الثانية" => Some(2),
        "ثلاث" | "ثلاثة" | "ثالث" | "الثالثة" => Some(3),
        "الثلاث" | "الثلاثة" => Some(3),
        "اربع" | "أربع" | "اربعة" | "أربعة" | "رابع" | "الرابعة" => Some(4),
        "الاربع" | "الأربع" | "الاربعة" | "الأربعة" => Some(4),
        "خمس" | "خمسة" | "خامس" | "الخامسة" => Some(5),
        "الخمس" | "الخمسة" => Some(5),
        "ست" | "ستة" | "سادس" | "السادسة" => Some(6),
        "الست" | "الستة" => Some(6),
        "سبع" | "سبعة" | "سابع" | "السابعة" => Some(7),
        "السبع" | "السبعة" => Some(7),
        "ثمان" | "ثماني" | "ثمانية" | "ثامن" | "الثامنة" => Some(8),
        "الثمان" | "الثماني" | "الثمانية" => Some(8),
        "تسع" | "تسعة" | "تاسع" | "التاسعة" => Some(9),
        "التسع" | "التسعة" => Some(9),
        _ => None,
    }
}

fn ar_tens_word(s: &str) -> Option<u32> {
    match s {
        "عشر" | "عشرة" | "عشرون" | "عشرين" => Some(10),
        "ثلاثون" | "ثلاثين" => Some(30),
        "اربعون" | "أربعون" | "اربعين" | "أربعين" => Some(40),
        "خمسون" | "خمسين" => Some(50),
        _ => None,
    }
}

fn ar_number_0_59(s: &str) -> Option<u32> {
    let s = s.trim();
    if let Ok(v) = s.parse::<u32>() {
        return (v <= 59).then_some(v);
    }
    if let Some(v) = ar_month_num(s) {
        return (v <= 59).then_some(v);
    }
    if let Some(v) = ar_hour_ordinal_feminine(s) {
        return (v <= 59).then_some(v);
    }
    if s == "احد عشر" || s == "أحد عشر" || s == "الحادية عشر" || s == "الحادية عشرة" {
        return Some(11);
    }
    if s == "اثنا عشر" || s == "اثنى عشر" || s == "الثانية عشر" || s == "الثانية عشرة" {
        return Some(12);
    }
    if let Some((a, b)) = s.split_once(" و") {
        let aa = a.trim();
        let bb = b.trim();
        let ones = ar_ones_word(aa)?;
        let tens = ar_tens_word(bb)?;
        let v = ones + tens;
        return (v <= 59).then_some(v);
    }
    if let Some((a, b)) = s.split_once(' ') {
        let aa = a.trim();
        let bb = b.trim();
        if (bb == "عشر" || bb == "عشرة") && (3..=9).contains(&ar_ones_word(aa).unwrap_or(0)) {
            let v = 10 + ar_ones_word(aa)?;
            return Some(v);
        }
    }
    if let Some(rest) = s.strip_prefix('و') {
        let rest = rest.trim();
        if !rest.is_empty() {
            return ar_number_0_59(rest);
        }
    }
    None
}

fn ar_adjust_hour(hour12_or_24: u32, part: &str) -> Option<u32> {
    let part = part.trim();
    if hour12_or_24 > 23 || hour12_or_24 == 0 {
        return None;
    }
    let is_pm = matches!(
        part,
        "مساء" | "مساءً" | "عصرا" | "العصر" | "بعد الظهر" | "بعد العصر" | "العشاء" | "هذه الليلة" | "بعد المغرب" | "قبل المغرب"
    );
    let is_noonish = matches!(part, "ظهرا" | "ظهرًا");
    if hour12_or_24 > 12 {
        return Some(hour12_or_24);
    }
    if is_pm {
        if hour12_or_24 == 12 {
            Some(12)
        } else {
            Some(hour12_or_24 + 12)
        }
    } else if is_noonish {
        if hour12_or_24 == 12 {
            Some(12)
        } else {
            Some(hour12_or_24 + 12)
        }
    } else {
        Some(hour12_or_24)
    }
}

fn ar_grain_word(s: &str) -> Option<Grain> {
    match s {
        "يوم" | "ايام" | "أيام" | "يومين" => Some(Grain::Day),
        "اسبوع" | "أسبوع" | "اسابيع" | "أسابيع" | "اسبوعين" | "أسبوعين" => Some(Grain::Week),
        "شهر" | "اشهر" | "أشهر" | "شهرين" => Some(Grain::Month),
        "سنة" | "سنين" | "سنوات" | "سنتين" => Some(Grain::Year),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ar)".to_string(),
            pattern: vec![regex("حالا|ال[آا]ن|في هذه اللحظة")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ar)".to_string(),
            pattern: vec![regex("اليوم")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ar)".to_string(),
            pattern: vec![regex("غد[اً]?|بكرا|بكرة")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ar)".to_string(),
            pattern: vec![regex("[أا]مس|البارحة")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "tonight (ar)".to_string(),
            pattern: vec![regex("هذه الليلة|الليلة")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(TimeData::new(TimeForm::PartOfDay(super::PartOfDay::Night))),
                ))))
            }),
        },
        Rule {
            name: "first of feb (ar)".to_string(),
            pattern: vec![regex("في اول شباط|الاول من شباط|الأول من شباط|الاول من شهر شباط|الأول من شهر شباط")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "day of week (ar)".to_string(),
            pattern: vec![regex("(ال)?[اإ]ثنين|(ال)?ثلاثاء?|(ال)?[اأ]ربعاء?|(ال)?خميس|(ال)?جمع[ةه]|(ال)?سبت|(ال)?[اأ]حد")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.contains("اثنين") {
                    0
                } else if s.contains("ثلاثاء") {
                    1
                } else if s.contains("اربعاء") || s.contains("أربعاء") {
                    2
                } else if s.contains("خميس") {
                    3
                } else if s.contains("جمع") {
                    4
                } else if s.contains("سبت") {
                    5
                } else if s.contains("احد") || s.contains("أحد") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "mid march (ar)".to_string(),
            pattern: vec![regex("نص شهر ثلاث|منتصف اذار|في نصف شهر مارس")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 15,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "month name (ar)".to_string(),
            pattern: vec![regex("يناير|كانون الثاني|فبراير|شباط|مارس|آذار|اذار|أبريل|ابريل|نيسان|مايو|أيار|ايار|يونيو|حزيران|يوليو|تموز|أغسطس|اغسطس|آب|اب|سبتمبر|أيلول|ايلول|أكتوبر|اكتوبر|تشرين الأول|تشرين الاول|نوفمبر|تشرين الثاني|ديسمبر|كانون الأول|كانون الاول")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => mm.group(0)?,
                    _ => return None,
                };
                let month = ar_month_name(m)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "first of march (ar)".to_string(),
            pattern: vec![regex("الاول من اذار|الأول من اذار|الاول من آذار|الأول من آذار")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "christmas (ar)".to_string(),
            pattern: vec![regex("عيد الميلاد|(?:يوم |عطل[ةه] )?((ال)?كري?سماس)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("christmas".to_string(), None))))),
        },
        Rule {
            name: "christmas eve (ar)".to_string(),
            pattern: vec![regex("(ليل[ةه] )((ال)?كري?سماس)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("christmas eve".to_string(), None))))),
        },
        Rule {
            name: "new year eve (ar)".to_string(),
            pattern: vec![regex("(ليل[ةه] )(ر[اأ]س السن[ةه])")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("new year's eve".to_string(), None))))),
        },
        Rule {
            name: "new year day (ar)".to_string(),
            pattern: vec![regex("(يوم |عطل[ةه] )?(ر[اأ]س السن[ةه])")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("new year's day".to_string(), None))))),
        },
        Rule {
            name: "valentine (ar)".to_string(),
            pattern: vec![regex("(عيد|يوم|عطل[ةه])((ال)?حب|(ال)?فالنتا?ين)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("valentine's day".to_string(), None))))),
        },
        Rule {
            name: "halloween (ar)".to_string(),
            pattern: vec![regex("(عيد |يوم |عطل[ةه] )?((ال)?هالوي?ين)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("halloween".to_string(), None))))),
        },
        Rule {
            name: "eid al-adha (ar)".to_string(),
            pattern: vec![regex("عيد الأضحى|عيد الاضحى")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("eid al-adha".to_string(), None))))),
        },
        Rule {
            name: "eid al-fitr (ar)".to_string(),
            pattern: vec![regex("عيد الفطر")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("eid al-fitr".to_string(), None))))),
        },
        Rule {
            name: "easter (ar)".to_string(),
            pattern: vec![regex("عيد الفصح")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("easter".to_string(), None))))),
        },
        Rule {
            name: "islamic new year (ar)".to_string(),
            pattern: vec![regex("رأس السنة الهجرية|راس السنة الهجرية")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("islamic new year".to_string(), None))))),
        },
        Rule {
            name: "ramadan (ar)".to_string(),
            pattern: vec![regex("رمضان")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("ramadan".to_string(), None))))),
        },
        Rule {
            name: "1 مارس (ar)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+مارس")],
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
            name: "في الاول من مارس (ar)".to_string(),
            pattern: vec![regex("في\\s+الاول\\s+من\\s+مارس|في\\s+الأول\\s+من\\s+مارس")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "بداية شهر 3 (ar)".to_string(),
            pattern: vec![regex("بداية شهر\\s*3")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "الرابع من ابريل (ar)".to_string(),
            pattern: vec![regex("الرابع من ابريل|الرابع من أبريل")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day: 4, year: None })))),
        },
        Rule {
            name: "الرابع من نيسان (ar)".to_string(),
            pattern: vec![regex("الرابع من نيسان")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day: 4, year: None })))),
        },
        Rule {
            name: "4 ابريل (ar)".to_string(),
            pattern: vec![regex("4\\s+ابريل|4\\s+أبريل")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day: 4, year: None })))),
        },
        Rule {
            name: "الثالث عشرة من شباط (ar)".to_string(),
            pattern: vec![regex("الثالث عشرة من شباط|الثالث عشر من شباط")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 13, year: None })))),
        },
        Rule {
            name: "الاسبوع الماضي (ar)".to_string(),
            pattern: vec![regex("الاسبوع الماضي|الأسبوع الماضي")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "الاسبوع السابق (ar)".to_string(),
            pattern: vec![regex("الاسبوع السابق|الأسبوع السابق")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "الاسبوع المنصرم (ar)".to_string(),
            pattern: vec![regex("الاسبوع المنصرم|الأسبوع المنصرم")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "قبل اسبوع (ar)".to_string(),
            pattern: vec![regex("قبل اسبوع|قبل أسبوع")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "من اسبوع (ar)".to_string(),
            pattern: vec![regex("من اسبوع|من أسبوع")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "هذا الاسبوع (ar)".to_string(),
            pattern: vec![regex("هذا الاسبوع|هذا الأسبوع")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "<cycle> this|last|next (ar)".to_string(),
            pattern: vec![regex("(ال)?(اسبوع|أسبوع|شهر|سنة|سنه|عام)\\s+(الحالي|القادم|التالي|المقبل|الجاي|السابق|الماضي|الماضى|الماض|الفائت|الفايت|المنصرم)")],
            production: Box::new(|nodes| {
                let (g, rel) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let grain = match g {
                    "اسبوع" | "أسبوع" => Grain::Week,
                    "شهر" => Grain::Month,
                    "سنة" | "سنه" | "عام" => Grain::Year,
                    _ => return None,
                };
                if rel == "الحالي" {
                    Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(grain))))
                } else if matches!(rel, "القادم" | "التالي" | "المقبل" | "الجاي") {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: 1 })))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: -1 })))
                }
            }),
        },
        Rule {
            name: "this|last <cycle> (ar)".to_string(),
            pattern: vec![regex("(هذا|هذه|اخر|آخر)\\s+(ال)?(اسبوع|أسبوع|شهر|سنة|سنه|عام)")],
            production: Box::new(|nodes| {
                let (rel, g) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?),
                    _ => return None,
                };
                let grain = match g {
                    "اسبوع" | "أسبوع" => Grain::Week,
                    "شهر" => Grain::Month,
                    "سنة" | "سنه" | "عام" => Grain::Year,
                    _ => return None,
                };
                if rel == "هذا" || rel == "هذه" {
                    Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(grain))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: -1 })))
                }
            }),
        },
        Rule {
            name: "<ordinal day> of month number (ar)".to_string(),
            pattern: vec![regex("(اول|أول|ثاني|ثالث|رابع|خامس|سادس|سابع|ثامن|تاسع|عاشر)\\s+يوم\\s+من\\s+شهر\\s+([0-9]{1,2}|واحد|الأول|الاول|اثنين|إثنين|اثنان|ثلاثة|أربعة|اربعة|خمسة|ستة|سبعة|ثمانية|تسعة|عشرة|أحد عشر|احد عشر|اثنا عشر|اثنى عشر)")],
            production: Box::new(|nodes| {
                let (d, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?),
                    _ => return None,
                };
                let day = ar_ordinal_day(d)?;
                let month = ar_month_num(m)?;
                if !(1..=12).contains(&month) {
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
            name: "<ordinal> week of <month> <year> (ar)".to_string(),
            pattern: vec![regex("(اول|أول|ثاني|ثالث|رابع)\\s+اسبوع\\s+بشهر\\s+(يناير|كانون الثاني|فبراير|شباط|مارس|آذار|اذار|أبريل|ابريل|نيسان|مايو|أيار|ايار|يونيو|حزيران|يوليو|تموز|أغسطس|اغسطس|آب|اب|سبتمبر|أيلول|ايلول|أكتوبر|اكتوبر|تشرين الأول|تشرين الاول|نوفمبر|تشرين الثاني|ديسمبر|كانون الأول|كانون الاول)\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (ord, mon, yr) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let n = ar_ordinal_day(ord)? as i32;
                let month = ar_month_name(mon)?;
                let year = yr.parse::<i32>().ok()?;
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Month(month))),
                    Box::new(TimeData::new(TimeForm::Year(year))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime {
                    n,
                    grain: Grain::Week,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "the <ordinal> week of month number year (ar)".to_string(),
            pattern: vec![regex("(ال)?اسبوع\\s+(ال)?(اول|أول|ثاني|ثالث|رابع)\\s+من\\s+شهر\\s+([0-9]{1,2}|واحد|الأول|الاول|اثنين|إثنين|اثنان|ثلاثة|أربعة|اربعة|خمسة|ستة|سبعة|ثمانية|تسعة|عشرة|أحد عشر|احد عشر|اثنا عشر|اثنى عشر)\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (ord, mon, yr) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(3)?, m.group(4)?, m.group(5)?),
                    _ => return None,
                };
                let n = ar_ordinal_day(ord)? as i32;
                let month = ar_month_num(mon)?;
                let year = yr.parse::<i32>().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Month(month))),
                    Box::new(TimeData::new(TimeForm::Year(year))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime {
                    n,
                    grain: Grain::Week,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "last day of month year (ar)".to_string(),
            pattern: vec![regex("(اخر|آخر)\\s+يوم\\s+بشهر\\s+([0-9]{1,2}|واحد|الأول|الاول|اثنين|إثنين|اثنان|ثلاثة|أربعة|اربعة|خمسة|ستة|سبعة|ثمانية|تسعة|عشرة|أحد عشر|احد عشر|اثنا عشر|اثنى عشر)\\s+سنة\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (mon, yr) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let month = ar_month_num(mon)?;
                let year = yr.parse::<i32>().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Month(month))),
                    Box::new(TimeData::new(TimeForm::Year(year))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::NthLastDayOfTime {
                    n: 1,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "last week of month year (ar)".to_string(),
            pattern: vec![regex("الاسبوع\\s+الاخير\\s+في\\s+الشهر\\s+([0-9]{1,2}|الأول|الاول|الثاني|الثالث|الرابع|الخامس|السادس|السابع|الثامن|التاسع|العاشر|الحادي عشر|الثاني عشر|واحد|اثنين|إثنين|اثنان|ثلاثة|أربعة|اربعة|خمسة|ستة|سبعة|ثمانية|تسعة|عشرة|أحد عشر|احد عشر|اثنا عشر|اثنى عشر)\\s+سنة\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (mon, yr) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let month = ar_month_num(mon)?;
                let year = yr.parse::<i32>().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Month(month))),
                    Box::new(TimeData::new(TimeForm::Year(year))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::LastCycleOfTime {
                    grain: Grain::Week,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "first <day-of-week> of month (ar)".to_string(),
            pattern: vec![regex("(الاثنين|الإثنين|اثنين|إثنين|الثلاثاء|الثلاثا|ثلاثاء|ثلاثا|الاربعاء|الأربعاء|اربعاء|أربعاء|الخميس|خميس|الجمعة|جمعه|جمعة|السبت|سبت|الاحد|الأحد|احد|أحد)\\s+(ال)?(اول|أول)\\s+من\\s+شهر\\s+(يناير|كانون الثاني|فبراير|شباط|مارس|آذار|اذار|أبريل|ابريل|نيسان|مايو|أيار|ايار|يونيو|حزيران|يوليو|تموز|أغسطس|اغسطس|آب|اب|سبتمبر|أيلول|ايلول|أكتوبر|اكتوبر|تشرين الأول|تشرين الاول|نوفمبر|تشرين الثاني|ديسمبر|كانون الأول|كانون الاول)")],
            production: Box::new(|nodes| {
                let (dow_s, mon_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(4)?),
                    _ => return None,
                };
                let dow = ar_day_of_week(dow_s)?;
                let month = ar_month_name(mon_s)?;
                let base = TimeData::new(TimeForm::Month(month));
                Some(TokenData::Time(TimeData::new(TimeForm::NthDOWOfTime {
                    n: 1,
                    dow,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "nth <day-of-week> in <month> year (ar)".to_string(),
            pattern: vec![regex("(اول|أول|ثان|ثاني|ثالث|رابع)\\s+(الاثنين|الإثنين|اثنين|إثنين|الثلاثاء|الثلاثا|ثلاثاء|ثلاثا|الاربعاء|الأربعاء|اربعاء|أربعاء|الخميس|خميس|الجمعة|جمعه|جمعة|السبت|سبت|الاحد|الأحد|احد|أحد)\\s+في\\s+(يناير|كانون الثاني|فبراير|شباط|مارس|آذار|اذار|أبريل|ابريل|نيسان|مايو|أيار|ايار|يونيو|حزيران|يوليو|تموز|أغسطس|اغسطس|آب|اب|سبتمبر|أيلول|ايلول|أكتوبر|اكتوبر|تشرين الأول|تشرين الاول|نوفمبر|تشرين الثاني|ديسمبر|كانون الأول|كانون الاول)\\s+لعام\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (ord_s, dow_s, mon_s, year_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?, m.group(3)?, m.group(4)?),
                    _ => return None,
                };
                let n = ar_ordinal_n(ord_s)?;
                let dow = ar_day_of_week(dow_s)?;
                let month = ar_month_name(mon_s)?;
                let year = year_s.parse::<i32>().ok()?;
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Month(month))),
                    Box::new(TimeData::new(TimeForm::Year(year))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::NthDOWOfTime {
                    n,
                    dow,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "second wed in month year short (ar)".to_string(),
            pattern: vec![regex("(ثان|ثاني)\\s+اربعا\\s+في\\s+(أكتوبر|اكتوبر|تشرين الأول|تشرين الاول)\\s+لعام\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (mon_s, year_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let month = ar_month_name(mon_s)?;
                let year = year_s.parse::<i32>().ok()?;
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Month(month))),
                    Box::new(TimeData::new(TimeForm::Year(year))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::NthDOWOfTime {
                    n: 2,
                    dow: 2,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "hour am/pm (ar)".to_string(),
            pattern: vec![regex("الساعة\\s*(\\d{1,2})\\s*(الصبح|الفجر|صباحا|صباحًا|مساء|مساءً|ليلا|ليلاً|عصرا|العصر|بعد الظهر|بعد العصر|العشاء|بعد المغرب|ظهرا|ظهرًا)")],
            production: Box::new(|nodes| {
                let (h, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let hour = ar_adjust_hour(hour, part)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Hour(hour, false))))
            }),
        },
        Rule {
            name: "hour am/pm word-number (ar)".to_string(),
            pattern: vec![regex("الساعة\\s*(واحد|واحدة|اثنين|إثنين|اثنان|ثلاثة|ثلاث|أربعة|اربعة|اربع|أربع|خمسة|خمس|ستة|ست|سبعة|سبع|ثمانية|ثماني|ثمان|تسعة|تسع|عشرة|أحد عشر|احد عشر|اثنا عشر|اثنى عشر)\\s*(الصبح|الفجر|صباحا|صباحًا|مساء|مساءً|ليلا|ليلاً|عصرا|العصر|بعد الظهر|بعد العصر|العشاء|بعد المغرب|ظهرا|ظهرًا)")],
            production: Box::new(|nodes| {
                let (h, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let hour = ar_number_0_59(h)?;
                let hour = ar_adjust_hour(hour, part)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Hour(hour, false))))
            }),
        },
        Rule {
            name: "at the <ordinal hour> am/pm (ar)".to_string(),
            pattern: vec![regex("(عند\\s+)?الساعة\\s+(الاولى|الأولى|الثانية|الثالثة|الرابعة|الخامسة|السادسة|السابعة|الثامنة|التاسعة|العاشرة|الحادية عشرة|الثانية عشرة)\\s*(الصبح|الفجر|صباحا|صباحًا|مساء|مساءً|ليلا|ليلاً|عصرا|العصر|بعد الظهر|بعد العصر|العشاء|بعد المغرب|ظهرا|ظهرًا)")],
            production: Box::new(|nodes| {
                let (h, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let hour = ar_hour_ordinal_feminine(h)?;
                let hour = ar_adjust_hour(hour, part)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Hour(hour, false))))
            }),
        },
        Rule {
            name: "<ordinal hour> am/pm (ar)".to_string(),
            pattern: vec![regex("(الاولى|الأولى|الثانية|الثالثة|الرابعة|الخامسة|السادسة|السابعة|الثامنة|التاسعة|العاشرة|الحادية عشرة|الثانية عشرة)\\s*(فجرا|الفجر|الصبح|صباحا|صباحًا|مساء|مساءً|ليلا|ليلاً|هذه الليلة|عصرا|العصر|بعد الظهر|بعد العصر|العشاء|بعد المغرب|ظهرا|ظهرًا)")],
            production: Box::new(|nodes| {
                let (h, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let hour = ar_hour_ordinal_feminine(h)?;
                let hour = ar_adjust_hour(hour, part)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Hour(hour, false))))
            }),
        },
        Rule {
            name: "hh:mm with daypart (ar)".to_string(),
            pattern: vec![regex("(\\d{1,2}):(\\d{2})\\s*(الصبح|الفجر|صباحا|صباحًا|قبل الظهر|ظهرا|ظهرًا|بعد الظهر|عصرا|العصر|مساء|مساءً|ليلا|ليلاً|العشاء|بعد المغرب)")],
            production: Box::new(|nodes| {
                let (h, m, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?, mm.group(3)?),
                    _ => return None,
                };
                let hour_raw = h.parse::<u32>().ok()?;
                let minute = m.parse::<u32>().ok()?;
                if minute > 59 {
                    return None;
                }
                let hour = ar_adjust_hour(hour_raw, part)?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "hour and minute with daypart (ar)".to_string(),
            pattern: vec![regex("(?:عند\\s+)?(?:الساعة\\s+)?([اأإآء-ي\\d\\s]+?)\\s*و\\s*([اأإآء-ي\\d\\s]+?)\\s*(?:دقيقة|دقائق)?\\s*(الصبح|الفجر|فجرا|صباحا|صباحًا|قبل الظهر|ظهرا|ظهرًا|بعد الظهر|عصرا|العصر|مساء|مساءً|ليلا|ليلاً|العشاء|بعد المغرب)")],
            production: Box::new(|nodes| {
                let (h, m, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?, mm.group(3)?),
                    _ => return None,
                };
                let hour_raw = ar_number_0_59(h.trim())?;
                if !(1..=12).contains(&hour_raw) {
                    return None;
                }
                let minute = ar_number_0_59(m.trim())?;
                if minute > 59 {
                    return None;
                }
                let hour = ar_adjust_hour(hour_raw, part)?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "hour and quarter/third/half (ar)".to_string(),
            pattern: vec![regex("(?:الساعة\\s+)?([اأإآء-ي\\d\\s]+?)\\s*و\\s*(ربع|ثلث|نصف)\\s*(الصبح|الفجر|صباحا|صباحًا|قبل الظهر|ظهرا|ظهرًا|بعد الظهر|عصرا|العصر|مساء|مساءً|ليلا|ليلاً|العشاء|بعد المغرب)")],
            production: Box::new(|nodes| {
                let (h, frac, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?, mm.group(3)?),
                    _ => return None,
                };
                let hour_raw = ar_number_0_59(h.trim())?;
                if !(1..=12).contains(&hour_raw) {
                    return None;
                }
                let minute = match frac {
                    "ربع" => 15,
                    "ثلث" => 20,
                    "نصف" => 30,
                    _ => return None,
                };
                let hour = ar_adjust_hour(hour_raw, part)?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "hour minus quarter/third (ar)".to_string(),
            pattern: vec![regex("(?:الساعة\\s+)?([اأإآء-ي\\d\\s]+?)\\s*(?:الا|إلا)\\s*(ربع|ربعا|ثلث|ثلثا)\\s*(الصبح|هذا الصباح|الفجر|صباحا|صباحًا|قبل الظهر|ظهرا|ظهرًا|بعد الظهر|عصرا|العصر|مساء|مساءً|ليلا|ليلاً|العشاء|بعد المغرب|قبل المغرب)")],
            production: Box::new(|nodes| {
                let (h, frac, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?, mm.group(3)?),
                    _ => return None,
                };
                let mut hour_raw = ar_number_0_59(h.trim())?;
                if !(1..=12).contains(&hour_raw) {
                    return None;
                }
                let minute = match frac {
                    "ربع" | "ربعا" => 45,
                    "ثلث" | "ثلثا" => 40,
                    _ => return None,
                };
                hour_raw = if hour_raw == 1 { 12 } else { hour_raw - 1 };
                let hour = ar_adjust_hour(hour_raw, part)?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "about hour (ar)".to_string(),
            pattern: vec![regex("(?:بحدود|حوالي)\\s+الساعة\\s+([اأإآء-ي\\d\\s]+)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => mm.group(1)?,
                    _ => return None,
                };
                let hour = ar_number_0_59(h.trim())?;
                if !(1..=23).contains(&hour) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Hour(hour, true))))
            }),
        },
        Rule {
            name: "hour and forty minutes (ar)".to_string(),
            pattern: vec![regex("الساعة\\s+([اأإآء-ي\\d\\s]+?)\\s+و?(?:اربعون|أربعون)\\s+دقيقة\\s*(الصبح|الفجر|صباحا|صباحًا|قبل الظهر|ظهرا|ظهرًا|بعد الظهر|بعد العصر|عصرا|العصر|مساء|مساءً|ليلا|ليلاً|العشاء|بعد المغرب|قبل المغرب)")],
            production: Box::new(|nodes| {
                let (h, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?),
                    _ => return None,
                };
                let hour_raw = ar_number_0_59(h.trim())?;
                if !(1..=12).contains(&hour_raw) {
                    return None;
                }
                let hour = ar_adjust_hour(hour_raw, part)?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 40, false))))
            }),
        },
        Rule {
            name: "early <month> (ar)".to_string(),
            pattern: vec![regex("(في\\s+)?(اوائل|أوائل)\\s+(?:شهر\\s+)?(يناير|كانون الثاني|فبراير|شباط|مارس|آذار|اذار|أبريل|ابريل|نيسان|مايو|أيار|ايار|يونيو|حزيران|يوليو|تموز|أغسطس|اغسطس|آب|اب|سبتمبر|أيلول|ايلول|أكتوبر|اكتوبر|تشرين الأول|تشرين الاول|نوفمبر|تشرين الثاني|ديسمبر|كانون الأول|كانون الاول)")],
            production: Box::new(|nodes| {
                let mon = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => mm.group(3)?,
                    _ => return None,
                };
                let month = ar_month_name(mon)?;
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: true,
                    target: Box::new(TimeForm::Month(month)),
                })))
            }),
        },
        Rule {
            name: "late <month> (ar)".to_string(),
            pattern: vec![regex("(في\\s+)?(اواخر|أواخر)\\s+(?:شهر\\s+)?(يناير|كانون الثاني|فبراير|شباط|مارس|آذار|اذار|أبريل|ابريل|نيسان|مايو|أيار|ايار|يونيو|حزيران|يوليو|تموز|أغسطس|اغسطس|آب|اب|سبتمبر|أيلول|ايلول|أكتوبر|اكتوبر|تشرين الأول|تشرين الاول|نوفمبر|تشرين الثاني|ديسمبر|كانون الأول|كانون الاول)")],
            production: Box::new(|nodes| {
                let mon = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => mm.group(3)?,
                    _ => return None,
                };
                let month = ar_month_name(mon)?;
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::Month(month)),
                })))
            }),
        },
        Rule {
            name: "plural n cycles next (ar)".to_string(),
            pattern: vec![regex("(?:ال)?(اسابيع|أسابيع|ايام|أيام|اشهر|أشهر|سنوات|سنين)\\s+([اأإآء-ي\\d\\s]+?)\\s+(القادمة|التالي(?:ة)?|المقبلة|الجاية)")],
            production: Box::new(|nodes| {
                let (g, n) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?),
                    _ => return None,
                };
                let grain = match g {
                    "اسابيع" | "أسابيع" => Grain::Week,
                    "ايام" | "أيام" => Grain::Day,
                    "اشهر" | "أشهر" => Grain::Month,
                    "سنوات" | "سنين" => Grain::Year,
                    _ => return None,
                };
                let n = ar_number_0_59(n)? as i64;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "weeks n next (ar)".to_string(),
            pattern: vec![regex("(?:الاسابيع|الأسابيع)\\s+(الثلاثة|ثلاثة|الثلاث|ثلاث|الأربعة|الاربعة|أربعة|اربعة|الخمسة|خمسة|الستة|ستة|السبعة|سبعة|الثمانية|ثمانية|التسعة|تسعة)\\s+القادمة")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => mm.group(1)?,
                    _ => return None,
                };
                let n = ar_number_0_59(n)? as i64;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain: Grain::Week,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "in/within dual duration (ar)".to_string(),
            pattern: vec![regex("(خلال|في\\s+غضون|بعد)\\s+(يومين|اسبوعين|أسبوعين|شهرين|سنتين)")],
            production: Box::new(|nodes| {
                let cyc = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => mm.group(2)?,
                    _ => return None,
                };
                let grain = ar_grain_word(cyc)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: 2, grain })))
            }),
        },
        Rule {
            name: "in/within n duration (ar)".to_string(),
            pattern: vec![regex("(خلال|في\\s+غضون|بعد)\\s+([اأإآء-ي\\d\\s]+?)\\s*(يوم|ايام|أيام|اسبوع|أسبوع|اسابيع|أسابيع|شهر|اشهر|أشهر|سنة|سنين|سنوات)")],
            production: Box::new(|nodes| {
                let (n, cyc) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(2)?, mm.group(3)?),
                    _ => return None,
                };
                let grain = ar_grain_word(cyc)?;
                let n = ar_number_0_59(n.trim())? as i64;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
        Rule {
            name: "<month> dd-dd interval (ar)".to_string(),
            pattern: vec![regex("(يناير|كانون الثاني|فبراير|شباط|مارس|آذار|اذار|أبريل|ابريل|نيسان|مايو|أيار|ايار|يونيو|حزيران|يوليو|تموز|أغسطس|اغسطس|آب|اب|سبتمبر|أيلول|ايلول|أكتوبر|اكتوبر|تشرين الأول|تشرين الاول|نوفمبر|تشرين الثاني|ديسمبر|كانون الأول|كانون الاول)\\s*(\\d{1,2})\\s*[-–]\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (mon, d1, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?, mm.group(3)?),
                    _ => return None,
                };
                let month = ar_month_name(mon)?;
                let d1 = d1.parse::<u32>().ok()?;
                let d2 = d2.parse::<u32>().ok()?;
                if !(1..=31).contains(&d1) || !(1..=31).contains(&d2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month, day: d1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: d2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "dd-dd <month> interval (ar)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*(?:الى|إلى|[-–])\\s*(\\d{1,2})\\s*(يناير|كانون الثاني|فبراير|شباط|مارس|آذار|اذار|أبريل|ابريل|نيسان|مايو|أيار|ايار|يونيو|حزيران|يوليو|تموز|أغسطس|اغسطس|آب|اب|سبتمبر|أيلول|ايلول|أكتوبر|اكتوبر|تشرين الأول|تشرين الاول|نوفمبر|تشرين الثاني|ديسمبر|كانون الأول|كانون الاول)")],
            production: Box::new(|nodes| {
                let (d1, d2, mon) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?, mm.group(3)?),
                    _ => return None,
                };
                let month = ar_month_name(mon)?;
                let d1 = d1.parse::<u32>().ok()?;
                let d2 = d2.parse::<u32>().ok()?;
                if !(1..=31).contains(&d1) || !(1..=31).contains(&d2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month, day: d1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: d2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "from dd to dd <month> interval (ar)".to_string(),
            pattern: vec![regex("من\\s*(\\d{1,2})\\s*(?:الى|إلى|[-–])\\s*(\\d{1,2})\\s*(يناير|كانون الثاني|فبراير|شباط|مارس|آذار|اذار|أبريل|ابريل|نيسان|مايو|أيار|ايار|يونيو|حزيران|يوليو|تموز|أغسطس|اغسطس|آب|اب|سبتمبر|أيلول|ايلول|أكتوبر|اكتوبر|تشرين الأول|تشرين الاول|نوفمبر|تشرين الثاني|ديسمبر|كانون الأول|كانون الاول)")],
            production: Box::new(|nodes| {
                let (d1, d2, mon) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?, mm.group(3)?),
                    _ => return None,
                };
                let month = ar_month_name(mon)?;
                let d1 = d1.parse::<u32>().ok()?;
                let d2 = d2.parse::<u32>().ok()?;
                if !(1..=31).contains(&d1) || !(1..=31).contains(&d2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month, day: d1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: d2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "from dd-dd of month number interval (ar)".to_string(),
            pattern: vec![regex("من\\s*(\\d{1,2})\\s*[-–]\\s*(\\d{1,2})\\s*من\\s*الشهر\\s*([0-9]{1,2}|الأول|الاول|الثاني|الثالث|الرابع|الخامس|السادس|السابع|الثامن|التاسع|العاشر|الحادي عشر|الثاني عشر)")],
            production: Box::new(|nodes| {
                let (d1, d2, mon) = match &nodes[0].token_data {
                    TokenData::RegexMatch(mm) => (mm.group(1)?, mm.group(2)?, mm.group(3)?),
                    _ => return None,
                };
                let month = ar_month_num(mon)?;
                let d1 = d1.parse::<u32>().ok()?;
                let d2 = d2.parse::<u32>().ok()?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&d1) || !(1..=31).contains(&d2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month, day: d1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: d2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "three weeks next literal (ar)".to_string(),
            pattern: vec![regex("الاسابيع الثلاثة القادمة|الأسابيع الثلاثة القادمة")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: 3,
                    grain: Grain::Week,
                    past: false,
                    interval: true,
                })))
            }),
        },
    ]);
    rules
}
