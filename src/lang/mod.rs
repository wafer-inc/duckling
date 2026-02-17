pub mod en;

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::dimensions;
use crate::locale::{Lang, Locale, Region};
use crate::types::{DimensionKind, Rule};

/// Get rules for a given language and set of dimensions.
/// Rules are cached after first compilation to avoid repeated regex compilation.
pub fn rules_for(locale: Locale, dims: &[DimensionKind]) -> &'static [Rule] {
    let cache = rule_cache();
    let key = CacheKey::new(locale.lang, locale.region, dims);

    if let Some(rules) = cache.lock().unwrap().get(&key).copied() {
        return rules;
    }

    let built = build_rules(locale, dims);
    let leaked: &'static [Rule] = Box::leak(built.into_boxed_slice());

    let mut guard = cache.lock().unwrap();
    guard.entry(key).or_insert(leaked)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    lang: Lang,
    region: Option<Region>,
    dims: Vec<DimensionKind>,
}

impl CacheKey {
    fn new(lang: Lang, region: Option<Region>, dims: &[DimensionKind]) -> Self {
        let mut normalized = dims.to_vec();
        normalized.sort_by_key(|d| *d as usize);
        normalized.dedup();
        Self {
            lang,
            region,
            dims: normalized,
        }
    }
}

fn rule_cache() -> &'static Mutex<HashMap<CacheKey, &'static [Rule]>> {
    static CACHE: OnceLock<Mutex<HashMap<CacheKey, &'static [Rule]>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn build_rules(locale: Locale, dims: &[DimensionKind]) -> Vec<Rule> {
    let needed = collect_needed_dims(locale.lang, dims);
    match locale.region {
        Some(region) => {
            let mut rules = common_rules(&needed);
            rules.extend(lang_rules(locale.lang, &needed));
            rules.extend(locale_rules(locale.lang, region, &needed));
            rules
        }
        None => {
            let mut rules = common_rules(&needed);
            rules.extend(default_rules(locale.lang, &needed));
            rules
        }
    }
}

fn collect_needed_dims(lang: Lang, dims: &[DimensionKind]) -> Vec<DimensionKind> {
    let mut needed: Vec<DimensionKind> = Vec::new();
    for dim in dims {
        add_with_deps(*dim, &mut needed);
    }
    if needed.is_empty() {
        match lang {
            Lang::EN => en::supported_dimensions(),
            Lang::AF => vec![DimensionKind::Numeral],
            Lang::AR => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Temperature,
                DimensionKind::Quantity,
                DimensionKind::Volume,
            ],
            Lang::ES => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Ordinal,
                DimensionKind::Quantity,
                DimensionKind::Temperature,
                DimensionKind::TimeGrain,
                DimensionKind::Duration,
                DimensionKind::Time,
                DimensionKind::Volume,
            ],
            Lang::BG => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Duration,
                DimensionKind::Ordinal,
                DimensionKind::TimeGrain,
                DimensionKind::Time,
            ],
            Lang::BN => vec![DimensionKind::Numeral],
            Lang::CA => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Duration,
                DimensionKind::Ordinal,
                DimensionKind::Temperature,
                DimensionKind::TimeGrain,
                DimensionKind::Volume,
            ],
            Lang::CS => vec![DimensionKind::Numeral, DimensionKind::Distance],
            Lang::DE => vec![
                DimensionKind::Numeral,
                DimensionKind::Distance,
                DimensionKind::Duration,
                DimensionKind::Email,
                DimensionKind::Ordinal,
                DimensionKind::TimeGrain,
                DimensionKind::Volume,
            ],
            Lang::FI => vec![DimensionKind::Numeral],
            Lang::DA => vec![DimensionKind::Numeral, DimensionKind::Ordinal],
            Lang::EL => vec![DimensionKind::Numeral, DimensionKind::Ordinal],
            Lang::ET => vec![DimensionKind::Numeral, DimensionKind::Ordinal],
            Lang::FA => vec![DimensionKind::Numeral],
            Lang::HE => vec![
                DimensionKind::Numeral,
                DimensionKind::Ordinal,
                DimensionKind::AmountOfMoney,
            ],
            Lang::HI => vec![
                DimensionKind::Numeral,
                DimensionKind::Ordinal,
                DimensionKind::Temperature,
            ],
            Lang::ID => vec![
                DimensionKind::Numeral,
                DimensionKind::Ordinal,
                DimensionKind::AmountOfMoney,
            ],
            Lang::JA => vec![
                DimensionKind::Numeral,
                DimensionKind::Ordinal,
                DimensionKind::Temperature,
            ],
            Lang::KA => vec![
                DimensionKind::Numeral,
                DimensionKind::Ordinal,
                DimensionKind::AmountOfMoney,
            ],
            Lang::KM => vec![
                DimensionKind::Numeral,
                DimensionKind::Ordinal,
                DimensionKind::Distance,
                DimensionKind::Quantity,
                DimensionKind::Temperature,
                DimensionKind::Volume,
            ],
            Lang::KN => vec![DimensionKind::Numeral],
            Lang::KO => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Ordinal,
                DimensionKind::Distance,
                DimensionKind::Quantity,
                DimensionKind::Temperature,
                DimensionKind::Volume,
            ],
            Lang::LO => vec![DimensionKind::Numeral],
            Lang::VI => vec![
                DimensionKind::Numeral,
                DimensionKind::Ordinal,
                DimensionKind::AmountOfMoney,
            ],
            Lang::ZH => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Ordinal,
                DimensionKind::Distance,
                DimensionKind::Quantity,
                DimensionKind::Temperature,
                DimensionKind::Volume,
            ],
            Lang::GA => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Duration,
                DimensionKind::Ordinal,
                DimensionKind::Temperature,
                DimensionKind::TimeGrain,
                DimensionKind::Volume,
            ],
            Lang::HR => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Ordinal,
                DimensionKind::Quantity,
                DimensionKind::Temperature,
                DimensionKind::Volume,
            ],
            Lang::MN => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Duration,
                DimensionKind::Ordinal,
                DimensionKind::Quantity,
                DimensionKind::Temperature,
                DimensionKind::TimeGrain,
                DimensionKind::Volume,
            ],
            Lang::MY => vec![DimensionKind::Numeral],
            Lang::NB => vec![
                DimensionKind::AmountOfMoney,
                DimensionKind::Numeral,
                DimensionKind::Ordinal,
            ],
            Lang::NE => vec![DimensionKind::Numeral],
            Lang::FR => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Duration,
                DimensionKind::Email,
                DimensionKind::Ordinal,
                DimensionKind::Quantity,
                DimensionKind::Temperature,
                DimensionKind::TimeGrain,
                DimensionKind::Volume,
            ],
            Lang::IT => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Email,
                DimensionKind::Ordinal,
                DimensionKind::Temperature,
                DimensionKind::Volume,
            ],
            Lang::IS => vec![DimensionKind::Numeral, DimensionKind::Email],
            Lang::NL => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Duration,
                DimensionKind::Ordinal,
                DimensionKind::Quantity,
                DimensionKind::TimeGrain,
                DimensionKind::Volume,
            ],
            Lang::PT => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Ordinal,
                DimensionKind::Quantity,
                DimensionKind::Temperature,
                DimensionKind::TimeGrain,
                DimensionKind::Volume,
            ],
            Lang::RO => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Duration,
                DimensionKind::Ordinal,
                DimensionKind::Quantity,
                DimensionKind::Temperature,
                DimensionKind::TimeGrain,
                DimensionKind::Volume,
            ],
            Lang::RU => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Ordinal,
                DimensionKind::Distance,
                DimensionKind::Quantity,
                DimensionKind::Volume,
            ],
            Lang::SV => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Duration,
                DimensionKind::Ordinal,
                DimensionKind::TimeGrain,
            ],
            Lang::SW => vec![DimensionKind::Numeral],
            Lang::SK => vec![DimensionKind::Numeral],
            Lang::HU => vec![DimensionKind::Numeral, DimensionKind::Ordinal],
            Lang::ML => vec![DimensionKind::Numeral, DimensionKind::Ordinal],
            Lang::PL => vec![DimensionKind::Numeral, DimensionKind::Ordinal],
            Lang::TA => vec![DimensionKind::Numeral, DimensionKind::Ordinal],
            Lang::TE => vec![DimensionKind::Numeral],
            Lang::TH => vec![DimensionKind::Numeral],
            Lang::TR => vec![
                DimensionKind::Numeral,
                DimensionKind::AmountOfMoney,
                DimensionKind::Distance,
                DimensionKind::Duration,
                DimensionKind::Ordinal,
                DimensionKind::Temperature,
                DimensionKind::TimeGrain,
                DimensionKind::Volume,
            ],
            Lang::UK => vec![DimensionKind::Numeral, DimensionKind::Ordinal],
        }
    } else {
        needed
    }
}

fn add_with_deps(dim: DimensionKind, needed: &mut Vec<DimensionKind>) {
    if needed.contains(&dim) {
        return;
    }
    for dep in dimensions::dimension_dependencies(dim) {
        add_with_deps(dep, needed);
    }
    needed.push(dim);
}

fn common_rules(needed: &[DimensionKind]) -> Vec<Rule> {
    let mut rules = Vec::new();
    for dim in needed {
        match dim {
            DimensionKind::Numeral => {
                rules.extend(crate::dimensions::numeral::en::common_rules())
            }
            DimensionKind::Distance => {
                rules.extend(crate::dimensions::distance::en::common_rules())
            }
            DimensionKind::Duration => {
                rules.extend(crate::dimensions::duration::en::common_rules())
            }
            DimensionKind::AmountOfMoney => {
                rules.extend(crate::dimensions::amount_of_money::en::common_rules())
            }
            // Mirrors Duckling.Rules.Common currently present in Rust implementation.
            DimensionKind::Email => rules.extend(crate::dimensions::email::rules::rules()),
            DimensionKind::PhoneNumber => {
                rules.extend(crate::dimensions::phone_number::rules::rules())
            }
            DimensionKind::Url => rules.extend(crate::dimensions::url::rules::rules()),
            DimensionKind::CreditCardNumber => {
                rules.extend(crate::dimensions::credit_card_number::rules::rules())
            }
            _ => {}
        }
    }
    rules
}

fn default_rules(lang: Lang, needed: &[DimensionKind]) -> Vec<Rule> {
    match lang {
        Lang::EN => en::default_rules(needed),
        _ => lang_rules(lang, needed),
    }
}

fn lang_rules(lang: Lang, needed: &[DimensionKind]) -> Vec<Rule> {
    let mut pre_time_rules: Vec<Rule> = Vec::new();
    if needed.contains(&DimensionKind::Time) {
        match lang {
            Lang::AR => pre_time_rules.extend(crate::dimensions::time::ar::rules()),
            Lang::CA => pre_time_rules.extend(crate::dimensions::time::ca::rules()),
            Lang::DA => pre_time_rules.extend(crate::dimensions::time::da::rules()),
            Lang::DE => pre_time_rules.extend(crate::dimensions::time::de::rules()),
            Lang::EL => pre_time_rules.extend(crate::dimensions::time::el::rules()),
            Lang::FR => pre_time_rules.extend(crate::dimensions::time::fr::rules()),
            Lang::GA => pre_time_rules.extend(crate::dimensions::time::ga::rules()),
            Lang::HE => pre_time_rules.extend(crate::dimensions::time::he::rules()),
            Lang::HR => pre_time_rules.extend(crate::dimensions::time::hr::rules()),
            Lang::HU => pre_time_rules.extend(crate::dimensions::time::hu::rules()),
            Lang::IT => pre_time_rules.extend(crate::dimensions::time::it::rules()),
            Lang::JA => pre_time_rules.extend(crate::dimensions::time::ja::rules()),
            Lang::KA => pre_time_rules.extend(crate::dimensions::time::ka::rules()),
            Lang::KO => pre_time_rules.extend(crate::dimensions::time::ko::rules()),
            Lang::NB => pre_time_rules.extend(crate::dimensions::time::nb::rules()),
            Lang::NL => pre_time_rules.extend(crate::dimensions::time::nl::rules()),
            Lang::PL => pre_time_rules.extend(crate::dimensions::time::pl::rules()),
            Lang::PT => pre_time_rules.extend(crate::dimensions::time::pt::rules()),
            Lang::RO => pre_time_rules.extend(crate::dimensions::time::ro::rules()),
            Lang::RU => pre_time_rules.extend(crate::dimensions::time::ru::rules()),
            Lang::SV => pre_time_rules.extend(crate::dimensions::time::sv::rules()),
            Lang::TR => pre_time_rules.extend(crate::dimensions::time::tr::rules()),
            Lang::UK => pre_time_rules.extend(crate::dimensions::time::uk::rules()),
            Lang::VI => pre_time_rules.extend(crate::dimensions::time::vi::rules()),
            Lang::ZH => pre_time_rules.extend(crate::dimensions::time::zh::rules()),
            _ => {}
        }
    }

    let mut rules = match lang {
        Lang::EN => en::lang_rules(needed),
        Lang::AF => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::af::rules());
                }
            }
            rules
        }
        Lang::AR => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::ar::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::ar::rules());
                    }
                    DimensionKind::Ordinal => {
                        rules.extend(crate::dimensions::ordinal::ar::rules());
                    }
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::ar::rules());
                    }
                    DimensionKind::Quantity => {
                        rules.extend(crate::dimensions::quantity::ar::rules());
                    }
                    DimensionKind::Duration => {
                        rules.extend(crate::dimensions::duration::ar::rules());
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::ar::rules());
                    }
                    DimensionKind::Volume => {
                        rules.extend(crate::dimensions::volume::ar::rules());
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::ES => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::es::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::es::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::es::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::es::rules()),
                    DimensionKind::Quantity => rules.extend(crate::dimensions::quantity::es::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::es::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::es::rules())
                    }
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::es::rules()),
                    DimensionKind::Time => rules.extend(crate::dimensions::time::es::rules()),
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::es::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::BG => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::bg::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::bg::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::bg::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::bg::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::bg::rules()),
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::bg::rules())
                    }
                    DimensionKind::Time => rules.extend(crate::dimensions::time::bg::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::BN => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::bn::rules());
                }
            }
            rules
        }
        Lang::CA => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::ca::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::ca::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::ca::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::ca::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::ca::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::ca::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::ca::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::ca::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::CS => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::cs::rules()),
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::cs::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::DE => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::de::rules()),
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::de::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::de::rules()),
                    DimensionKind::Email => rules.extend(crate::dimensions::email::de::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::de::rules()),
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::de::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::de::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::DA => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::da::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::da::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::EL => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => {
                        rules.extend(crate::dimensions::numeral::el::rules());
                    }
                    DimensionKind::Ordinal => {
                        rules.extend(crate::dimensions::ordinal::el::rules());
                    }
                    DimensionKind::Duration => {
                        rules.extend(crate::dimensions::duration::el::rules());
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::el::rules());
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::ET => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::et::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::et::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::FI => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::fi::rules());
                }
            }
            rules
        }
        Lang::FA => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::fa::rules());
                }
            }
            rules
        }
        Lang::HE => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::he::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::he::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::he::rules())
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::HI => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::hi::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::hi::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::hi::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::hi::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::hi::rules())
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::ID => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::id::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::id::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::id::rules())
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::JA => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::ja::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::ja::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::ja::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::ja::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::ja::rules())
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::KA => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::ka::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::ka::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::ka::rules())
                    }
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::ka::rules()),
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::ka::rules())
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::KM => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::km::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::km::rules()),
                    DimensionKind::Distance => {
                        rules.extend(crate::dimensions::distance::km::rules())
                    }
                    DimensionKind::Quantity => {
                        rules.extend(crate::dimensions::quantity::km::rules())
                    }
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::km::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::km::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::KN => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::kn::rules());
                }
            }
            rules
        }
        Lang::KO => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::ko::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::ko::rules())
                    }
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::ko::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::ko::rules()),
                    DimensionKind::Distance => {
                        rules.extend(crate::dimensions::distance::ko::rules())
                    }
                    DimensionKind::Quantity => {
                        rules.extend(crate::dimensions::quantity::ko::rules())
                    }
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::ko::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::ko::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::ko::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::LO => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::lo::rules());
                }
            }
            rules
        }
        Lang::VI => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::vi::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::vi::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::vi::rules())
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::ZH => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::zh::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::zh::rules())
                    }
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::zh::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::zh::rules()),
                    DimensionKind::Distance => {
                        rules.extend(crate::dimensions::distance::zh::rules())
                    }
                    DimensionKind::Quantity => {
                        rules.extend(crate::dimensions::quantity::zh::rules())
                    }
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::zh::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::zh::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::zh::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::GA => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::ga::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::ga::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::ga::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::ga::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::ga::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::ga::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::ga::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::ga::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::HR => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::hr::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::hr::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::hr::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::hr::rules()),
                    DimensionKind::Quantity => rules.extend(crate::dimensions::quantity::hr::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::hr::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::hr::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::MN => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::mn::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::mn::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::mn::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::mn::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::mn::rules()),
                    DimensionKind::Quantity => rules.extend(crate::dimensions::quantity::mn::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::mn::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::mn::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::mn::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::MY => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::my::rules());
                }
            }
            rules
        }
        Lang::NB => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::nb::rules())
                    }
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::nb::rules()),
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::nb::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::nb::rules()),
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::nb::rules())
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::NE => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::ne::rules());
                }
            }
            rules
        }
        Lang::FR => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::fr::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::fr::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::fr::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::fr::rules()),
                    DimensionKind::Email => rules.extend(crate::dimensions::email::fr::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::fr::rules()),
                    DimensionKind::Quantity => rules.extend(crate::dimensions::quantity::fr::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::fr::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::fr::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::fr::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::IT => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::it::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::it::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::it::rules()),
                    DimensionKind::Email => rules.extend(crate::dimensions::email::it::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::it::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::it::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::it::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::IS => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::is::rules()),
                    DimensionKind::Email => rules.extend(crate::dimensions::email::rules::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::NL => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::nl::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::nl::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::nl::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::nl::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::nl::rules()),
                    DimensionKind::Quantity => rules.extend(crate::dimensions::quantity::nl::rules()),
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::nl::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::nl::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::PT => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::pt::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::pt::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::pt::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::pt::rules()),
                    DimensionKind::Quantity => rules.extend(crate::dimensions::quantity::pt::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::pt::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::pt::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::pt::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::RO => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::ro::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::ro::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::ro::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::ro::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::ro::rules()),
                    DimensionKind::Quantity => rules.extend(crate::dimensions::quantity::ro::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::ro::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::ro::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::ro::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::RU => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::ru::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::ru::rules())
                    }
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::ru::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::ru::rules()),
                    DimensionKind::Distance => {
                        rules.extend(crate::dimensions::distance::ru::rules())
                    }
                    DimensionKind::Quantity => {
                        rules.extend(crate::dimensions::quantity::ru::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::ru::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::ru::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::SV => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::sv::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::sv::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::sv::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::sv::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::sv::rules()),
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::sv::rules())
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::SK => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::sk::rules());
                }
            }
            rules
        }
        Lang::SW => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::sw::rules());
                }
            }
            rules
        }
        Lang::HU => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::hu::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::hu::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::hu::rules()),
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::hu::rules())
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::ML => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::ml::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::ml::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::PL => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::pl::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::pl::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::pl::rules()),
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::pl::rules())
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::TA => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::ta::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::ta::rules()),
                    _ => {}
                }
            }
            rules
        }
        Lang::TE => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::te::rules());
                }
            }
            rules
        }
        Lang::TH => {
            let mut rules = Vec::new();
            for dim in needed {
                if *dim == DimensionKind::Numeral {
                    rules.extend(crate::dimensions::numeral::th::rules());
                }
            }
            rules
        }
        Lang::UK => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::uk::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::uk::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::uk::rules()),
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::uk::rules())
                    }
                    _ => {}
                }
            }
            rules
        }
        Lang::TR => {
            let mut rules = Vec::new();
            for dim in needed {
                match dim {
                    DimensionKind::Numeral => rules.extend(crate::dimensions::numeral::tr::rules()),
                    DimensionKind::AmountOfMoney => {
                        rules.extend(crate::dimensions::amount_of_money::tr::rules())
                    }
                    DimensionKind::Distance => rules.extend(crate::dimensions::distance::tr::rules()),
                    DimensionKind::Duration => rules.extend(crate::dimensions::duration::tr::rules()),
                    DimensionKind::Ordinal => rules.extend(crate::dimensions::ordinal::tr::rules()),
                    DimensionKind::Temperature => {
                        rules.extend(crate::dimensions::temperature::tr::rules())
                    }
                    DimensionKind::TimeGrain => {
                        rules.extend(crate::dimensions::time_grain::tr::rules())
                    }
                    DimensionKind::Volume => rules.extend(crate::dimensions::volume::tr::rules()),
                    _ => {}
                }
            }
            rules
        }
    };
    if !pre_time_rules.is_empty() {
        pre_time_rules.append(&mut rules);
        pre_time_rules
    } else {
        rules
    }
}

fn locale_rules(lang: Lang, region: Region, needed: &[DimensionKind]) -> Vec<Rule> {
    match lang {
        Lang::EN => en::locale_rules(Some(region), needed),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::locale::Lang;

    #[test]
    fn subset_dims_use_fewer_rules_than_all_dims() {
        let locale = Locale::new(Lang::EN, None);
        let all = rules_for(locale, &[]);
        let url_only = rules_for(locale, &[DimensionKind::Url]);
        assert!(!url_only.is_empty(), "expected URL rules to be loaded");
        assert!(
            url_only.len() < all.len(),
            "expected subset rules ({}) to be fewer than full rules ({})",
            url_only.len(),
            all.len()
        );
    }
}
