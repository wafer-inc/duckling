use super::{Direction, IntervalDirection, PartOfDay, TimeData, TimeForm};
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (el)".to_string(),
            pattern: vec![regex("τ[ώω]ρα")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (el)".to_string(),
            pattern: vec![regex("σ[ήη]μερα")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (el)".to_string(),
            pattern: vec![regex("α[ύυ]ριο")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "tonight (el)".to_string(),
            pattern: vec![regex("απ(ό|ο)ψε|σ(ή|η)μερα\\s+το\\s+βρ(ά|α)δυ")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))))),
        },
        Rule {
            name: "midnight (el)".to_string(),
            pattern: vec![regex("(τα\\s+)?μεσ(ά|α)νυχτα")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Hour(0, false))))),
        },
        Rule {
            name: "noon (el)".to_string(),
            pattern: vec![regex("(το\\s+)?μεσημ(έ|ε)ρι(ού)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Hour(12, false))))),
        },
        Rule {
            name: "yesterday (el)".to_string(),
            pattern: vec![regex("χθες|εχθ(έ|ε)ς")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day before yesterday / day after tomorrow (el)".to_string(),
            pattern: vec![regex("προχθ(έ|ε)ς|προχτ(έ|ε)ς|μεθα(ύ|υ)ριο")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.starts_with("πρ") {
                    Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))
                }
            }),
        },
        Rule {
            name: "day of week (el)".to_string(),
            pattern: vec![regex("δευτ(έρας?|\\.?)|τρ[ιί](της?|\\.?)|τετ(άρτης?|\\.?)|π[εέ]μ(πτης?|\\.?)|παρ(ασκευής?|\\.?)|σ[αά]β(β[αά]το[νυ]?|\\.?)|κυρ(ιακής?|\\.?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("δευ") {
                    0
                } else if s.contains("τρί") || s.contains("τρι") {
                    1
                } else if s.contains("τετ") {
                    2
                } else if s.contains("πέμ") || s.contains("πεμ") {
                    3
                } else if s.contains("παρασκευ") || s.starts_with("παρ") {
                    4
                } else if s.contains("σάβ") || s.contains("σαβ") {
                    5
                } else if s.contains("κυριακ") || s.starts_with("κυρ") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "day-of-month ordinal token (el)".to_string(),
            pattern: vec![dim(DimensionKind::Ordinal)],
            production: Box::new(|nodes| {
                let day = match &nodes[0].token_data {
                    TokenData::Ordinal(o) if (1..=31).contains(&o.value) => o.value as u32,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "the day-of-month ordinal (el)".to_string(),
            pattern: vec![regex("τ?η[νς]?\\s*([012]?\\d|30|31)η")],
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
            name: "date with flevary (el)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+φ(λεβ[άα]ρη|εβρουαρ[ίι]ου)(,\\s*δευτ)?")],
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
            name: "january abbr (el)".to_string(),
            pattern: vec![regex("ιαν")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(1))))),
        },
        Rule {
            name: "january colloquial (el)".to_string(),
            pattern: vec![regex("γενάρης|γεναρης|γενάρη")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(1))))),
        },
        Rule {
            name: "february variants (el)".to_string(),
            pattern: vec![regex("φεβ|φεβρουάριο|φεβρουαρίου|φλεβάρη")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(2))))),
        },
        Rule {
            name: "march variants (el)".to_string(),
            pattern: vec![regex("μάρτης|μαρτης|μάρτιος")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(3))))),
        },
        Rule {
            name: "march short (el)".to_string(),
            pattern: vec![regex("μάρτη|μαρτη")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(3))))),
        },
        Rule {
            name: "μαρτίου (el)".to_string(),
            pattern: vec![regex("μαρτίου|μαρτιου")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(3))))),
        },
        Rule {
            name: "μάρτιο (el)".to_string(),
            pattern: vec![regex("μάρτιο|μαρτιο|μάρτιον|μαρτιον")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(3))))),
        },
        Rule {
            name: "μαρ (el)".to_string(),
            pattern: vec![regex("μαρ\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(3))))),
        },
        Rule {
            name: "απρίλης (el)".to_string(),
            pattern: vec![regex("απρίλης|απριλης")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(4))))),
        },
        Rule {
            name: "απρ (el)".to_string(),
            pattern: vec![regex("απρ\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(4))))),
        },
        Rule {
            name: "μάη (el)".to_string(),
            pattern: vec![regex("μάη|μαη")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(5))))),
        },
        Rule {
            name: "μαϊου (el)".to_string(),
            pattern: vec![regex("μα[ϊι]ου")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(5))))),
        },
        Rule {
            name: "μάιο (el)".to_string(),
            pattern: vec![regex("μάιο|μαιο")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(5))))),
        },
        Rule {
            name: "ιούνιος (el)".to_string(),
            pattern: vec![regex("ιούνιος|ιουνιος")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(6))))),
        },
        Rule {
            name: "ιουνίου (el)".to_string(),
            pattern: vec![regex("ιουνίου|ιουνιου")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(6))))),
        },
        Rule {
            name: "ιούνη (el)".to_string(),
            pattern: vec![regex("ιούνη|ιουνη")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(6))))),
        },
        Rule {
            name: "ιούνιον (el)".to_string(),
            pattern: vec![regex("ιούνιον|ιουνιον")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(6))))),
        },
        Rule {
            name: "ιουν (el)".to_string(),
            pattern: vec![regex("ιουν\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(6))))),
        },
        Rule {
            name: "ιουλ (el)".to_string(),
            pattern: vec![regex("ιουλ\\.?|Ιουλ\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(7))))),
        },
        Rule {
            name: "ιούλη (el)".to_string(),
            pattern: vec![regex("ιούλη|ιουλη")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(7))))),
        },
        Rule {
            name: "ιούλιο (el)".to_string(),
            pattern: vec![regex("ιούλιο|ιουλιο")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(7))))),
        },
        Rule {
            name: "αυγ (el)".to_string(),
            pattern: vec![regex("αυγ\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(8))))),
        },
        Rule {
            name: "αύγουστο (el)".to_string(),
            pattern: vec![regex("αύγουστο|αυγουστο")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(8))))),
        },
        Rule {
            name: "σεπτ (el)".to_string(),
            pattern: vec![regex("σεπτ\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(9))))),
        },
        Rule {
            name: "Οκτ (el)".to_string(),
            pattern: vec![regex("οκτ([ωώ]βρ([ιί]ο([νυ]?)|η)ς?)?\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(10))))),
        },
        Rule {
            name: "νοεμ (el)".to_string(),
            pattern: vec![regex("νοεμ\\.?|νοέμβρης|νοεμβρης|νοέμβριος|νοεμβριος|νοέμβριο|νοεμβριο|νοέμβρη|νοεμβρη")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(11))))),
        },
        Rule {
            name: "δεκ (el)".to_string(),
            pattern: vec![regex("δεκ\\.?|δεκέμβρης|δεκεμβρης|δεκέμβριος|δεκεμβριος|δεκέμβριο|δεκεμβριο|δεκέμβρη|δεκεμβρη")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(12))))),
        },
        Rule {
            name: "year (el)".to_string(),
            pattern: vec![regex("\\b(\\d{4})\\b")],
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
            name: "evangelismos (el)".to_string(),
            pattern: vec![regex("ευαγγελισμ(ό|ο)ς της θεοτ(ό|ο)κου|ευαγγελισμο(ύ|υ) της θεοτ(ό|ο)κου")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 25, year: None })))),
        },
        Rule {
            name: "christmas (el)".to_string(),
            pattern: vec![regex("χριστο(ύ|υ)γεννα")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "christmas day".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "season (el)".to_string(),
            pattern: vec![regex("αυτ(ό|ο)\\s+το\\s+φθιν(ό|ο)πωρο|αυτο(ύ|υ)\\s+του\\s+φθινοπ(ώ|ο)ρου|φθιν(ό|ο)πωρο|καλοκα(ί|ι)ρι|χειμ(ώ|ο)να|(ά|α)νοιξη")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.contains("άνοι") || s.contains("ανοι") {
                    0
                } else if s.contains("καλοκα") {
                    1
                } else if s.contains("φθιν") {
                    2
                } else {
                    3
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "new year eve/day (el)".to_string(),
            pattern: vec![regex("παραμον(ή|ε)ς πρωτοχρονι(ά|α)ς|αν(ή|η)μερα πρωτοχρονι(ά|α)ς")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let name = if s.contains("παραμον") {
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
            name: "mother/father/revolution/halloween (el)".to_string(),
            pattern: vec![regex("η(μέρα| μέρα| μερα)? της μητ(έ|ε)ρας|η( μέρα|μέρα)? του πατ(έ|ε)ρα|η( περασμ(έ|ε)νη)? μέρα της επαν(ά|α)στασης|halloween|του\\s+αγ(ί|ι)ου\\s+βαλεντ(ί|ι)νου")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("μητ") {
                    return Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                        "mother's day".to_string(),
                        None,
                    ))));
                }
                if s.contains("πατ") {
                    return Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                        "father's day".to_string(),
                        None,
                    ))));
                }
                if s.contains("halloween") {
                    return Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                        "halloween".to_string(),
                        None,
                    ))));
                }
                if s.contains("βαλεντ") {
                    return Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                        month: 2,
                        day: 14,
                        year: None,
                    })));
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 25,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "one year after christmas (el)".to_string(),
            pattern: vec![regex("(ένα|μια|μία|\\d{1,3})\\s+χρ(ό|ο)νο\\s+μετ(ά|α)\\s+τα\\s+χριστο(ύ|υ)γεννα")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let years: i64 = match n {
                    "ένα" | "μια" | "μία" => 1,
                    _ => n.parse().ok()?,
                };
                let base = TimeData::new(TimeForm::Holiday("christmas day".to_string(), None));
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n: years,
                    grain: Grain::Year,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "this week (el)".to_string(),
            pattern: vec![regex("αυτ(ή|η)\\s+τη\\s+(βδομ(ά|α)δα|εβδομ(ά|α)δα)|τρ(έ|ε)χουσα\\s+εβδομ(ά|α)δα")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "prev/next week (el)".to_string(),
            pattern: vec![regex("(περασμ(έ|ε)νη|προηγο(ύ|υ)μενη)\\s+εβδομ(ά|α)δα|επ(ό|ο)μενη\\s+εβδομ(ά|α)δα")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("επ") {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))
                }
            }),
        },
        Rule {
            name: "prev/next month (el)".to_string(),
            pattern: vec![regex("τον\\s+προηγο(ύ|υ)μενο\\s+μ(ή|η)να|τον\\s+επ(ό|ο)μενο\\s+μ(ή|η)να")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("επ") {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))
                }
            }),
        },
        Rule {
            name: "this/prev/next year (el)".to_string(),
            pattern: vec![regex("αυτ(ή|η)\\s+τη\\s+χρονι(ά|α)|αυτ(ό|ο)\\s+το\\s+(έ|ε)τος|την\\s+περασμ(έ|ε)νη\\s+χρονι(ά|α)|την\\s+επ(ό|ο)μενη\\s+χρονι(ά|α)|τον\\s+επ(ό|ο)μενο\\s+χρ(ό|ο)νο|του\\s+χρ(ό|ο)νου|τουχρ(ό|ο)νου")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("αυτ") {
                    Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))
                } else if s.contains("επ") || s.contains("τουχρ") || s.contains("του χρ") {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))
                }
            }),
        },
        Rule {
            name: "φέτος/πέρσι (el)".to_string(),
            pattern: vec![regex("φ(έ|ε)τος|π(έ|ε)ρσι|π(έ|ε)ρυσι")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.starts_with("φ") {
                    Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                        grain: Grain::Year,
                        offset: -1,
                    })))
                }
            }),
        },
        Rule {
            name: "next weekday (el)".to_string(),
            pattern: vec![regex("(την\\s+)?επ(ό|ο)μενη\\s+(δευτ(έ|ε)ρα|τρ(ί|ι)τη|τετ(ά|α)ρτη|π(έ|ε)μπτη|παρασκευ(ή|η)|κυριακ(ή|η)|σαββατο)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(3)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("δευτ") {
                    0
                } else if s.starts_with("τρ") {
                    1
                } else if s.starts_with("τετ") {
                    2
                } else if s.starts_with("π") {
                    3
                } else if s.starts_with("παρασκ") {
                    4
                } else if s.starts_with("σαβ") {
                    5
                } else {
                    6
                };
                let mut td = TimeData::new(TimeForm::DayOfWeek(dow));
                td.direction = Some(Direction::Future);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "last weekend / sk (el)".to_string(),
            pattern: vec![regex("το\\s+(περασμ(έ|ε)νο|προηγο(ύ|υ)μενο)\\s+σαββατοκ(ύ|υ)ριακο|το\\s+(περασμ(έ|ε)νο|προηγο(ύ|υ)μενο)\\s+σκ")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: Grain::Week,
                    offset: -1,
                })))
            }),
        },
        Rule {
            name: "this weekend / sk (el)".to_string(),
            pattern: vec![regex("αυτ(ό|ο)\\s+το\\s+σαββατοκ(ύ|υ)ριακο|αυτ(ό|ο)\\s+το\\s+σκ")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "last/next n cycles (el)".to_string(),
            pattern: vec![regex("(τα|τις|οι)?\\s*(τελευτα(ί|ι)α|επ(ό|ο)μενα|επ(ό|ο)μενες|επ(ό|ο)μενοι)\\s+(\\d{1,3})\\s+(δε(ύ|υ)τερα|λεπτ(ά|α)|ώρες|μ(έ|ε)ρες|βδομ(ά|α)δες|μ(ή|η)νες|χρ(ό|ο)νια)")],
            production: Box::new(|nodes| {
                let (dir, n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?.to_lowercase(), m.group(5)?, m.group(6)?.to_lowercase()),
                    _ => return None,
                };
                let value: i64 = n.parse().ok()?;
                let grain = if unit.starts_with("δε") {
                    Grain::Second
                } else if unit.starts_with("λεπτ") {
                    Grain::Minute
                } else if unit.starts_with("ώ") || unit.starts_with("ω") {
                    Grain::Hour
                } else if unit.starts_with("μ") && unit.contains("ερ") {
                    Grain::Day
                } else if unit.contains("βδομ") {
                    Grain::Week
                } else if unit.starts_with("μ") && unit.contains("ην") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                let past = dir.starts_with("τελευτα");
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: value,
                    grain,
                    past,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "τα τελευταία 2 δεύτερα (el)".to_string(),
            pattern: vec![regex("τα\\s+τελευτα(ί|ι)α\\s+(ένα|μια|μία|δύο|δυο|τρία|τρεις|\\d{1,3})\\s+(δε(ύ|υ)τερα|λεπτ(ά|α)|ώρες|μ(έ|ε)ρες|βδομ(ά|α)δες|μ(ή|η)νες|χρ(ό|ο)νια)")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let value: i64 = match n {
                    "ένα" | "μια" | "μία" => 1,
                    "δύο" | "δυο" => 2,
                    "τρία" | "τρεις" => 3,
                    _ => n.parse().ok()?,
                };
                let grain = if unit.starts_with("δε") {
                    Grain::Second
                } else if unit.starts_with("λεπτ") {
                    Grain::Minute
                } else if unit.starts_with("ώ") || unit.starts_with("ω") {
                    Grain::Hour
                } else if unit.starts_with("μ") && unit.contains("ερ") {
                    Grain::Day
                } else if unit.contains("βδομ") {
                    Grain::Week
                } else if unit.starts_with("μ") && unit.contains("ην") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: value,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "τα επόμενα 3 δεύτερα (el)".to_string(),
            pattern: vec![regex("(τα|τις|οι)\\s+επ(ό|ο)μεν(α|ες|οι)\\s+(ένα|μια|μία|δύο|δυο|τρία|τρεις|\\d{1,3})\\s+(δε(ύ|υ)τερα|λεπτ(ά|α)|ώρες|μ(έ|ε)ρες|βδομ(ά|α)δες|μ(ή|η)νες|χρ(ό|ο)νια)")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(4)?, m.group(5)?.to_lowercase()),
                    _ => return None,
                };
                let value: i64 = match n {
                    "ένα" | "μια" | "μία" => 1,
                    "δύο" | "δυο" => 2,
                    "τρία" | "τρεις" => 3,
                    _ => n.parse().ok()?,
                };
                let grain = if unit.starts_with("δε") {
                    Grain::Second
                } else if unit.starts_with("λεπτ") {
                    Grain::Minute
                } else if unit.starts_with("ώ") || unit.starts_with("ω") {
                    Grain::Hour
                } else if unit.starts_with("μ") && unit.contains("ερ") {
                    Grain::Day
                } else if unit.contains("βδομ") {
                    Grain::Week
                } else if unit.starts_with("μ") && unit.contains("ην") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: value,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "η τελευταία 1 ώρα (el)".to_string(),
            pattern: vec![regex("η\\s+τελευτα(ί|ι)α\\s+(ένα|μια|μία|\\d{1,3})\\s+(ώρα|μ(έ|ε)ρα|βδομ(ά|α)δα|μ(ή|η)να|χρ(ό|ο)νο)")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let value: i64 = match n {
                    "ένα" | "μια" | "μία" => 1,
                    _ => n.parse().ok()?,
                };
                let grain = if unit.starts_with("ώ") || unit.starts_with("ω") {
                    Grain::Hour
                } else if unit.starts_with("μ") && unit.contains("ερ") {
                    Grain::Day
                } else if unit.contains("βδομ") {
                    Grain::Week
                } else if unit.starts_with("μ") && unit.contains("ην") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: value,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "τελευταίες 2 μέρες (el)".to_string(),
            pattern: vec![regex("(τελευτα(ί|ι)(ες?|οι|α)|περασμ(έ|ε)(νες?|νοι|να))\\s+(\\d{1,3})\\s+(μ(έ|ε)ρες|ώρες|λεπτ(ά|α)|βδομ(ά|α)δες|εβδομ(ά|α)δες|μ(ή|η)νες|χρ(ό|ο)νια)")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(6)?, m.group(7)?.to_lowercase()),
                    _ => return None,
                };
                let value: i64 = n.parse().ok()?;
                let grain = if unit.starts_with("λεπτ") {
                    Grain::Minute
                } else if unit.starts_with("ώ") || unit.starts_with("ω") {
                    Grain::Hour
                } else if unit.starts_with("μ") && unit.contains("ερ") {
                    Grain::Day
                } else if unit.contains("βδομ") {
                    Grain::Week
                } else if unit.starts_with("μ") && unit.contains("ην") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: value,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "επόμενες τρεις μέρες (el)".to_string(),
            pattern: vec![regex("επ(ό|ο)μεν(ες?|α|οι)\\s+(ένα|μια|μία|δύο|δυο|τρία|τρεις|\\d{1,3})\\s+(μ(έ|ε)ρες|ώρες|λεπτ(ά|α)|βδομ(ά|α)δες|εβδομ(ά|α)δες|μ(ή|η)νες|χρ(ό|ο)νια)")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(3)?, m.group(4)?.to_lowercase()),
                    _ => return None,
                };
                let value: i64 = match n {
                    "ένα" | "μια" | "μία" => 1,
                    "δύο" | "δυο" => 2,
                    "τρία" | "τρεις" => 3,
                    _ => n.parse().ok()?,
                };
                let grain = if unit.starts_with("λεπτ") {
                    Grain::Minute
                } else if unit.starts_with("ώ") || unit.starts_with("ω") {
                    Grain::Hour
                } else if unit.starts_with("μ") && unit.contains("ερ") {
                    Grain::Day
                } else if unit.contains("βδομ") {
                    Grain::Week
                } else if unit.starts_with("μ") && unit.contains("ην") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: value,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "επόμενες μερικές μέρες (el)".to_string(),
            pattern: vec![regex("επ(ό|ο)μεν(ες?|α)\\s+μερικ(έ|ε)ς\\s+μ(έ|ε)ρες")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: 3,
                    grain: Grain::Day,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "μέχρι το τέλος της ημέρας/μήνα (el)".to_string(),
            pattern: vec![regex("μ(έ|ε)χρι\\s+το\\s+τ(έ|ε)λος\\s+της\\s+ημ(έ|ε)ρας|μ(έ|ε)χρι\\s+το\\s+τ(έ|ε)λος\\s+του\\s+(επ(ό|ο)μενου\\s+)?μ(ή|η)να")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let target = if s.contains("ημ") {
                    TimeForm::GrainOffset { grain: Grain::Day, offset: 0 }
                } else if s.contains("επ") {
                    TimeForm::GrainOffset { grain: Grain::Month, offset: 1 }
                } else {
                    TimeForm::GrainOffset { grain: Grain::Month, offset: 0 }
                };
                let mut td = TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(target),
                });
                td.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "3πμ / 5μμ (el)".to_string(),
            pattern: vec![regex("στις?\\s*(\\d{1,2})(:(\\d{2}))?\\s*(πμ|μμ)|(\\d{1,2})(:(\\d{2}))?\\s*(πμ|μμ)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let htxt = m.group(1).or_else(|| m.group(5))?;
                let mtxt = m.group(3).or_else(|| m.group(7)).unwrap_or("0");
                let ap = m.group(4).or_else(|| m.group(8))?;
                let mut hour: u32 = htxt.parse().ok()?;
                let minute: u32 = mtxt.parse().ok()?;
                if ap == "μμ" && hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                if ap == "πμ" && hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "3 το πρωί / 8 το βράδυ (el)".to_string(),
            pattern: vec![regex("(\\d{1,2})(\\s+η\\s+ώρα)?\\s+το\\s+(πρω(ί|ι)|βρ(ά|α)δυ|απ(ό|ο)γευμα)")],
            production: Box::new(|nodes| {
                let (h, pod) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if (pod.contains("βρ") || pod.contains("απ")) && hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "3:18π / 3:18μ (el)".to_string(),
            pattern: vec![regex("(\\d{1,2}):(\\d{2})(π|μ)")],
            production: Box::new(|nodes| {
                let (h, m, ap) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if ap == "π" && hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "3 η ώρα μμ/πμ (el)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+η\\s+ώρα\\s+(μμ|πμ)")],
            production: Box::new(|nodes| {
                let (h, ap) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if ap == "μμ" && hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                if ap == "πμ" && hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "3 και κάτι μμ (el)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+και\\s+κ(ά|α)τι\\s+(μμ|πμ)")],
            production: Box::new(|nodes| {
                let (h, ap) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if ap == "μμ" && hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                if ap == "πμ" && hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "3 και τέταρτο/μισή μμ (el)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+και\\s+(τ(έ|ε)ταρτο|μισ(ή|η))\\s+(μμ|πμ)")],
            production: Box::new(|nodes| {
                let (h, part, ap) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?.to_lowercase(), rm.group(5)?),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if ap == "μμ" && hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                if ap == "πμ" && hour == 12 {
                    hour = 0;
                }
                let minute = if part.contains("ταρ") { 15 } else { 30 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "στις 3 και 15 (el)".to_string(),
            pattern: vec![regex("στις?\\s*(\\d{1,2})\\s+και\\s+(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
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
            name: "στις τρεις και είκοσι (el)".to_string(),
            pattern: vec![regex("στις?\\s*(μία|μια|δύο|δυο|τρεις|τέσσερις|τέσσερες|πέντε|έξι|επτά|εφτά|οκτώ|εννιά|δέκα|έντεκα|δώδεκα)\\s+και\\s+(δέκα|είκοσι|τριάντα|σαράντα|πενήντα)")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let hour = match h.as_str() {
                    "μία" | "μια" => 1,
                    "δύο" | "δυο" => 2,
                    "τρεις" => 3,
                    "τέσσερις" | "τέσσερες" => 4,
                    "πέντε" => 5,
                    "έξι" => 6,
                    "επτά" | "εφτά" => 7,
                    "οκτώ" => 8,
                    "εννιά" => 9,
                    "δέκα" => 10,
                    "έντεκα" => 11,
                    "δώδεκα" => 12,
                    _ => return None,
                };
                let minute = match m.as_str() {
                    "δέκα" => 10,
                    "είκοσι" => 20,
                    "τριάντα" => 30,
                    "σαράντα" => 40,
                    "πενήντα" => 50,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "στις τρεις και μισή μμ (el)".to_string(),
            pattern: vec![regex("στις?\\s*(μία|μια|δύο|δυο|τρεις|τέσσερις|τέσσερες|πέντε|έξι|επτά|εφτά|οκτώ|εννιά|δέκα|έντεκα|δώδεκα)\\s+και\\s+μισ(ή|η)\\s+(μμ|πμ)")],
            production: Box::new(|nodes| {
                let (h, ap) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(3)?),
                    _ => return None,
                };
                let mut hour: u32 = match h.as_str() {
                    "μία" | "μια" => 1,
                    "δύο" | "δυο" => 2,
                    "τρεις" => 3,
                    "τέσσερις" | "τέσσερες" => 4,
                    "πέντε" => 5,
                    "έξι" => 6,
                    "επτά" | "εφτά" => 7,
                    "οκτώ" => 8,
                    "εννιά" => 9,
                    "δέκα" => 10,
                    "έντεκα" => 11,
                    "δώδεκα" => 12,
                    _ => return None,
                };
                if ap == "μμ" && hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                if ap == "πμ" && hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 30, false))))
            }),
        },
        Rule {
            name: "τρεισήμισι μμ (el)".to_string(),
            pattern: vec![regex("τρεισ(ή|η)μισι\\s+(μμ|πμ)")],
            production: Box::new(|nodes| {
                let ap = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(2)?,
                    _ => return None,
                };
                let hour = if ap == "μμ" { 15 } else { 3 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 30, false))))
            }),
        },
        Rule {
            name: "330 μ.μ. (el)".to_string(),
            pattern: vec![regex("([1-9])(\\d{2})\\s*(μ\\.?μ\\.?|π\\.?μ\\.?)")],
            production: Box::new(|nodes| {
                let (h, m, ap) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if ap.starts_with('μ') && hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                if ap.starts_with('π') && hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "τρεις και μισή (el)".to_string(),
            pattern: vec![regex("(μία|μια|δύο|δυο|τρεις|τέσσερις|τέσσερες|πέντε|έξι|επτά|εφτά|οκτώ|εννιά|δέκα|έντεκα|δώδεκα)\\s+και\\s+μισ(ή|η)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let hour = match h.as_str() {
                    "μία" | "μια" => 1,
                    "δύο" | "δυο" => 2,
                    "τρεις" => 3,
                    "τέσσερις" | "τέσσερες" => 4,
                    "πέντε" => 5,
                    "έξι" => 6,
                    "επτά" | "εφτά" => 7,
                    "οκτώ" => 8,
                    "εννιά" => 9,
                    "δέκα" => 10,
                    "έντεκα" => 11,
                    "δώδεκα" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 30, true))))
            }),
        },
        Rule {
            name: "quarter before noon (el)".to_string(),
            pattern: vec![regex("ένα\\s+τ(έ|ε)ταρτο\\s+πριν\\s+το\\s+μεσημ(έ|ε)ρι")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(11, 45, false))))),
        },
        Rule {
            name: "tonight at 8 (el)".to_string(),
            pattern: vec![regex("απ(ό|ο)ψε\\s+στις\\s*(\\d{1,2})|σ(ή|η)μερα\\s+το\\s+βρ(ά|α)δυ\\s+στις\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2).or_else(|| m.group(4))?,
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "σε ένα/δύο <cycle> (el)".to_string(),
            pattern: vec![regex("σε\\s+(ένα|μια|μία|δύο|δυο|τρία|τρεις|\\d{1,3})\\s+(δε(ύ|υ)τερ(ο|α)?|λεπτ(ό|ο|ά|α)|ώρ(α|ες)|μ(έ|ε)ρ(α|ες)|βδομ(ά|α)δ(α|ες)|εβδομ(ά|α)δ(α|ες)|μ(ή|η)ν(α|ες)|χρ(ό|ο)ν(ο|ια))")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let value: i64 = match n {
                    "ένα" | "μια" | "μία" => 1,
                    "δύο" | "δυο" => 2,
                    "τρία" | "τρεις" => 3,
                    _ => n.parse().ok()?,
                };
                let grain = if unit.starts_with("δε") {
                    Grain::Second
                } else if unit.starts_with("λεπτ") {
                    Grain::Minute
                } else if unit.starts_with("ώ") || unit.starts_with("ω") {
                    Grain::Hour
                } else if unit.starts_with("μ") && unit.contains("ερ") {
                    Grain::Day
                } else if unit.contains("βδομ") {
                    Grain::Week
                } else if unit.starts_with("μ") && unit.contains("ην") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: value, grain })))
            }),
        },
        Rule {
            name: "πριν από <n> <cycle> (el)".to_string(),
            pattern: vec![regex("πριν\\s+απ(ό|ο)\\s+(ένα|μια|μία|δύο|δυο|τρία|τρεις|\\d{1,3})\\s+(χρ(ό|ο)νια|χρ(ό|ο)νο|εβδομ(ά|α)δες|εβδομ(ά|α)δα|βδομ(ά|α)δες|βδομ(ά|α)δα|μ(ή|η)νες|μ(ή|η)να|μ(έ|ε)ρες|μ(έ|ε)ρα|ώρες|ώρα|λεπτ(ά|α)|λεπτ(ό|ο)|δε(ύ|υ)τερα|δε(ύ|υ)τερο)")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let value: i64 = match n {
                    "ένα" | "μια" => 1,
                    "δύο" | "δυο" => 2,
                    "τρία" | "τρεις" => 3,
                    _ => n.parse().ok()?,
                };
                let grain = if unit.starts_with("δε") {
                    Grain::Second
                } else if unit.starts_with("λεπτ") {
                    Grain::Minute
                } else if unit.starts_with("ώ") || unit.starts_with("ω") {
                    Grain::Hour
                } else if unit.starts_with("μ") && unit.contains("ερ") {
                    Grain::Day
                } else if unit.contains("βδομ") {
                    Grain::Week
                } else if unit.starts_with("μ") && unit.contains("ην") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: value.checked_neg()?,
                    grain,
                })))
            }),
        },
        Rule {
            name: "<n> <cycle> πριν (el)".to_string(),
            pattern: vec![regex("(\\d{1,3})\\s+(χρ(ό|ο)νια|εβδομ(ά|α)δες|μ(ή|η)νες|μ(έ|ε)ρες|ώρες|λεπτ(ά|α)|δε(ύ|υ)τερα)\\s+πριν")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let value: i64 = n.parse().ok()?;
                let grain = if unit.starts_with("δε") {
                    Grain::Second
                } else if unit.starts_with("λεπτ") {
                    Grain::Minute
                } else if unit.starts_with("ώ") || unit.starts_with("ω") {
                    Grain::Hour
                } else if unit.starts_with("μ") && unit.contains("ερ") {
                    Grain::Day
                } else if unit.contains("βδομ") {
                    Grain::Week
                } else if unit.starts_with("μ") && unit.contains("ην") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: value.checked_neg()?,
                    grain,
                })))
            }),
        },
        Rule {
            name: "εδώ και <n> <cycle> (el)".to_string(),
            pattern: vec![regex("εδ(ώ|ω)\\s+και\\s+(ένα|μια|μία|δύο|δυο|τρία|τρεις|\\d{1,3})\\s+(χρ(ό|ο)νια|χρ(ό|ο)νο|εβδομ(ά|α)δες|εβδομ(ά|α)δα|βδομ(ά|α)δες|βδομ(ά|α)δα|μ(ή|η)νες|μ(ή|η)να|μ(έ|ε)ρες|μ(έ|ε)ρα|ώρες|ώρα|λεπτ(ά|α)|λεπτ(ό|ο)|δε(ύ|υ)τερα|δε(ύ|υ)τερο)")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let value: i64 = match n {
                    "ένα" | "μια" | "μία" => 1,
                    "δύο" | "δυο" => 2,
                    "τρία" | "τρεις" => 3,
                    _ => n.parse().ok()?,
                };
                let grain = if unit.starts_with("δε") {
                    Grain::Second
                } else if unit.starts_with("λεπτ") {
                    Grain::Minute
                } else if unit.starts_with("ώ") || unit.starts_with("ω") {
                    Grain::Hour
                } else if unit.starts_with("μ") && unit.contains("ερ") {
                    Grain::Day
                } else if unit.contains("βδομ") {
                    Grain::Week
                } else if unit.starts_with("μ") && unit.contains("ην") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: value.checked_neg()?,
                    grain,
                })))
            }),
        },
        Rule {
            name: "σε 1\" / σε 2' (el)".to_string(),
            pattern: vec![regex("σε\\s*(\\d{1,3})\\s*([\"'])")],
            production: Box::new(|nodes| {
                let (n, sym) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let value: i64 = n.parse().ok()?;
                let grain = if sym == "\"" {
                    Grain::Second
                } else {
                    Grain::Minute
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: value, grain })))
            }),
        },
        Rule {
            name: "20 λεπτά πριν τις 12 (el)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+λεπτ(ά|α)\\s+πριν\\s+τις\\s+(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (m, h) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?),
                    _ => return None,
                };
                let mins: u32 = m.parse().ok()?;
                let hour: u32 = h.parse().ok()?;
                if mins >= 60 || !(1..=24).contains(&hour) {
                    return None;
                }
                let out_hour = if hour == 24 { 23 } else { hour.checked_sub(1)? };
                let out_min = 60_u32.checked_sub(mins)?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_hour, out_min, false))))
            }),
        },
        Rule {
            name: "20 λεπτά μετά τις 12 (el)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+λεπτ(ά|α)\\s+μετ(ά|α)\\s+τις\\s+(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (m, h) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?),
                    _ => return None,
                };
                let mins: u32 = m.parse().ok()?;
                let hour: u32 = h.parse().ok()?;
                if mins >= 60 || hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, mins, false))))
            }),
        },
        Rule {
            name: "minutes after hour fallback (el)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*λεπτ.*\\s*μετ.*\\s*τις\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (m, h) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let mins: u32 = m.parse().ok()?;
                let hour: u32 = h.parse().ok()?;
                if mins >= 60 || hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, mins, false))))
            }),
        },
        Rule {
            name: "in a few minutes/days (el)".to_string(),
            pattern: vec![regex("σε\\s+μερικ(ά|α)\\s+λεπτ(ά|α)|σε\\s+μερικ(έ|ε)ς\\s+μ(έ|ε)ρες|σε\\s+μερικ(έ|ε)ς\\s+ώρες")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let (n, grain) = if s.contains("λεπτ") {
                    (3, Grain::Minute)
                } else if s.contains("ώρ") || s.contains("ωρ") {
                    (3, Grain::Hour)
                } else {
                    (3, Grain::Day)
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
        Rule {
            name: "in a quarter of an hour (el)".to_string(),
            pattern: vec![regex("σε\\s+ένα\\s+τ(έ|ε)ταρτο\\s+της\\s+ώρας")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: 15,
                    grain: Grain::Minute,
                })))
            }),
        },
        Rule {
            name: "σε 1/4ω (el)".to_string(),
            pattern: vec![regex("σε\\s*1/4\\s*ω")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: 15,
                    grain: Grain::Minute,
                })))
            }),
        },
        Rule {
            name: "σε 1ω (el)".to_string(),
            pattern: vec![regex("σε\\s*(\\d{1,3})\\s*ω")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let h: i64 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: h,
                    grain: Grain::Hour,
                })))
            }),
        },
        Rule {
            name: "σε μισή ώρα (el)".to_string(),
            pattern: vec![regex("σε\\s+(περ(ί|ι)που\\s+)?μισ(ή|η)\\s+ώρα")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: 30,
                    grain: Grain::Minute,
                })))
            }),
        },
        Rule {
            name: "σε 1/2ω (el)".to_string(),
            pattern: vec![regex("σε\\s*1/2\\s*ω")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: 30,
                    grain: Grain::Minute,
                })))
            }),
        },
        Rule {
            name: "σε 3/4ω (el)".to_string(),
            pattern: vec![regex("σε\\s*(3/4\\s*(ω|της\\s+ώρας)|τρία\\s+τ(έ|ε)ταρτα\\s+της\\s+ώρας)")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: 45,
                    grain: Grain::Minute,
                })))
            }),
        },
        Rule {
            name: "σε 2,5 ώρες (el)".to_string(),
            pattern: vec![regex("σε\\s*(\\d+),(\\d+)\\s+ώρες")],
            production: Box::new(|nodes| {
                let (h, frac) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let hh: i64 = h.parse().ok()?;
                let num: i64 = frac.parse().ok()?;
                let den = 10_i64.pow(frac.len() as u32);
                let minutes = hh.checked_mul(60)?.checked_add(num.checked_mul(60)?.checked_div(den)?)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: minutes,
                    grain: Grain::Minute,
                })))
            }),
        },
        Rule {
            name: "σε 2 και μισή ώρες (el)".to_string(),
            pattern: vec![regex("σε\\s*(\\d+)\\s+και\\s+μισ(ή|η)\\s+ώρες")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hh: i64 = h.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: hh.checked_mul(60)?.checked_add(30)?,
                    grain: Grain::Minute,
                })))
            }),
        },
    ]);
    rules
}
