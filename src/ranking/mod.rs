use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use crate::dimensions::time::TimeForm;
use crate::dimensions::time_grain::Grain;
use crate::locale::{Lang, Locale};
use crate::types::{DimensionKind, Entity, Node, TokenData};
use serde::{Deserialize, Serialize};

#[cfg(feature = "train")]
pub(crate) mod train;

pub(crate) type Feature = String;
pub(crate) type BagOfFeatures = HashMap<Feature, i32>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassData {
    #[serde(
        serialize_with = "serialize_f64_with_inf",
        deserialize_with = "deserialize_f64_with_inf"
    )]
    pub prior: f64,
    #[serde(
        serialize_with = "serialize_f64_with_inf",
        deserialize_with = "deserialize_f64_with_inf"
    )]
    pub unseen: f64,
    pub likelihoods: HashMap<Feature, f64>,
    pub n: i32,
}

fn serialize_f64_with_inf<S: serde::Serializer>(v: &f64, s: S) -> Result<S::Ok, S::Error> {
    if v.is_infinite() {
        if v.is_sign_negative() {
            s.serialize_str("-Infinity")
        } else {
            s.serialize_str("Infinity")
        }
    } else {
        s.serialize_f64(*v)
    }
}

fn deserialize_f64_with_inf<'de, D: serde::Deserializer<'de>>(d: D) -> Result<f64, D::Error> {
    use serde::de;

    struct F64Visitor;
    impl<'de> de::Visitor<'de> for F64Visitor {
        type Value = f64;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("f64 or infinity string")
        }
        fn visit_f64<E: de::Error>(self, v: f64) -> Result<f64, E> {
            Ok(v)
        }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<f64, E> {
            Ok(v as f64)
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<f64, E> {
            Ok(v as f64)
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<f64, E> {
            match v {
                "-Infinity" => Ok(f64::NEG_INFINITY),
                "Infinity" => Ok(f64::INFINITY),
                _ => Err(de::Error::custom(format!("unexpected string: {}", v))),
            }
        }
        fn visit_unit<E: de::Error>(self) -> Result<f64, E> {
            Ok(f64::NEG_INFINITY)
        }
    }
    d.deserialize_any(F64Visitor)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classifier {
    pub ok_data: ClassData,
    pub ko_data: ClassData,
}

/// Map from rule name to its trained Naive Bayes classifier.
pub type Classifiers = HashMap<String, Classifier>;

#[derive(Debug, Deserialize)]
struct JsonClassData {
    #[serde(deserialize_with = "deserialize_f64_with_inf")]
    prior: f64,
    #[serde(deserialize_with = "deserialize_f64_with_inf")]
    unseen: f64,
    likelihoods: HashMap<String, f64>,
    #[serde(default)]
    n: i32,
}

#[derive(Debug, Deserialize)]
struct JsonClassifier {
    ok_data: JsonClassData,
    #[serde(default)]
    ko_data: Option<JsonClassData>,
}

#[derive(Clone)]
struct Candidate {
    node: Node,
    score: f64,
    target: bool,
}

fn time_form_grain(form: &TimeForm) -> Option<Grain> {
    match form {
        TimeForm::Year(_) => Some(Grain::Year),
        TimeForm::Month(_) => Some(Grain::Month),
        TimeForm::Quarter(_) | TimeForm::QuarterYear(_, _) => Some(Grain::Quarter),
        TimeForm::DayOfWeek(_)
        | TimeForm::DayOfMonth(_)
        | TimeForm::DateMDY { .. }
        | TimeForm::Today
        | TimeForm::Tomorrow
        | TimeForm::Yesterday
        | TimeForm::DayAfterTomorrow
        | TimeForm::DayBeforeYesterday
        | TimeForm::Holiday(..)
        | TimeForm::Season(_)
        | TimeForm::Weekend => Some(Grain::Day),
        TimeForm::Hour(_, _) => Some(Grain::Hour),
        TimeForm::HourMinute(_, _, _) => Some(Grain::Minute),
        TimeForm::HourMinuteSecond(_, _, _) => Some(Grain::Second),
        TimeForm::RelativeGrain { grain, .. } => Some(*grain),
        TimeForm::PartOfDay(_) => Some(Grain::Hour),
        TimeForm::GrainOffset { grain, .. } => Some(*grain),
        TimeForm::NthGrain { grain, .. } => Some(*grain),
        TimeForm::Now => Some(Grain::Second),
        TimeForm::Composed(a, b) => time_form_grain(&a.form).or_else(|| time_form_grain(&b.form)),
        TimeForm::Interval(_, _, _) => Some(Grain::Hour),
        TimeForm::BeginEnd { target, .. } => time_form_grain(target),
        TimeForm::NthDOWOfTime { base, .. } => time_form_grain(&base.form),
        TimeForm::LastDOWOfTime { base, .. } => time_form_grain(&base.form),
        TimeForm::LastCycleOfTime { base, .. } => time_form_grain(&base.form),
        TimeForm::NDOWsFromTime { base, .. } => time_form_grain(&base.form),
        TimeForm::NthClosestToTime { base, .. } => time_form_grain(&base.form),
        TimeForm::NthGrainOfTime { base, .. } => time_form_grain(&base.form),
        TimeForm::NthLastDayOfTime { base, .. } => time_form_grain(&base.form),
        TimeForm::DurationAfter { grain, .. } => Some(*grain),
        TimeForm::NthLastCycleOfTime { grain, .. } => Some(*grain),
        TimeForm::AllGrain(g) | TimeForm::RestOfGrain(g) => Some(*g),
    }
}

pub(crate) fn extract_features(node: &Node) -> BagOfFeatures {
    let feat_rules = node
        .children
        .iter()
        .filter_map(|c| c.rule_name.as_ref())
        .fold(String::new(), |mut acc, rn| {
            acc.push_str(rn);
            acc
        });

    let mut grains = String::new();
    for child in &node.children {
        match &child.token_data {
            TokenData::Duration(d) => grains.push_str(d.grain.as_str()),
            TokenData::Time(t) => {
                if let Some(g) = time_form_grain(&t.form) {
                    grains.push_str(g.as_str());
                }
            }
            TokenData::TimeGrain(g) => grains.push_str(g.as_str()),
            _ => {}
        }
    }

    let mut out = HashMap::new();
    out.insert(feat_rules, 1);
    if !grains.is_empty() {
        out.insert(grains, 1);
    }
    out
}

fn ll(feats: &BagOfFeatures, class_data: &ClassData) -> f64 {
    class_data.prior
        + feats.iter().fold(0.0, |acc, (feat, x)| {
            let w = class_data
                .likelihoods
                .get(feat)
                .copied()
                .unwrap_or(class_data.unseen);
            acc + (*x as f64) * w
        })
}

fn score_node(classifiers: &Classifiers, node: &Node) -> f64 {
    let self_score = node
        .rule_name
        .as_ref()
        .and_then(|r| classifiers.get(r))
        .map(|c| ll(&extract_features(node), &c.ok_data))
        .unwrap_or(0.0);
    self_score
        + node
            .children
            .iter()
            .map(|c| score_node(classifiers, c))
            .sum::<f64>()
}

fn comp_range(a: &Node, b: &Node) -> Ordering {
    let starts = a.range.start.cmp(&b.range.start);
    let ends = a.range.end.cmp(&b.range.end);
    match starts {
        Ordering::Equal => ends,
        Ordering::Less => match ends {
            Ordering::Less => Ordering::Equal,
            _ => Ordering::Greater,
        },
        Ordering::Greater => match ends {
            Ordering::Greater => Ordering::Equal,
            _ => Ordering::Less,
        },
    }
}

fn same_dimension(a: &Node, b: &Node) -> bool {
    a.token_data.dimension_kind() == b.token_data.dimension_kind()
}

fn compare_candidate(a: &Candidate, b: &Candidate) -> Ordering {
    if same_dimension(&a.node, &b.node) {
        let starts = a.node.range.start.cmp(&b.node.range.start);
        let ends = a.node.range.end.cmp(&b.node.range.end);
        return match starts {
            Ordering::Equal => match ends {
                Ordering::Equal => a.score.partial_cmp(&b.score).unwrap_or(Ordering::Equal),
                z => z,
            },
            Ordering::Less => match ends {
                Ordering::Less => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Ordering::Greater => match ends {
                Ordering::Greater => Ordering::Equal,
                _ => Ordering::Less,
            },
        };
    }

    let cr = comp_range(&a.node, &b.node);
    if a.target == b.target {
        return cr;
    }
    if a.target && cr == Ordering::Greater {
        return Ordering::Greater;
    }
    if b.target && cr == Ordering::Less {
        return Ordering::Less;
    }
    Ordering::Equal
}

fn classifiers_for_locale(locale: &Locale) -> Classifiers {
    static EN_XX: OnceLock<Classifiers> = OnceLock::new();
    static AR_XX: OnceLock<Classifiers> = OnceLock::new();
    static EL_XX: OnceLock<Classifiers> = OnceLock::new();
    static ES_XX: OnceLock<Classifiers> = OnceLock::new();
    static PT_XX: OnceLock<Classifiers> = OnceLock::new();
    static TR_XX: OnceLock<Classifiers> = OnceLock::new();

    fn load(json: &str) -> Classifiers {
        let raw: HashMap<String, JsonClassifier> =
            serde_json::from_str(json).expect("failed to parse classifier JSON");
        raw.into_iter()
            .map(|(rule, jc)| {
                let ok_data = ClassData {
                    prior: jc.ok_data.prior,
                    unseen: jc.ok_data.unseen,
                    likelihoods: jc.ok_data.likelihoods,
                    n: jc.ok_data.n,
                };
                let ko_data = match jc.ko_data {
                    Some(ko) => ClassData {
                        prior: ko.prior,
                        unseen: ko.unseen,
                        likelihoods: ko.likelihoods,
                        n: ko.n,
                    },
                    None => ClassData {
                        prior: f64::NEG_INFINITY,
                        unseen: f64::NEG_INFINITY,
                        likelihoods: HashMap::new(),
                        n: 0,
                    },
                };
                (rule, Classifier { ok_data, ko_data })
            })
            .collect()
    }

    match locale.lang {
        Lang::EN => EN_XX
            .get_or_init(|| load(include_str!("../ranking_classifiers/en_xx.json")))
            .clone(),
        Lang::AR => AR_XX
            .get_or_init(|| load(include_str!("../ranking_classifiers/ar_xx.json")))
            .clone(),
        Lang::EL => EL_XX
            .get_or_init(|| load(include_str!("../ranking_classifiers/el_xx.json")))
            .clone(),
        Lang::ES => ES_XX
            .get_or_init(|| load(include_str!("../ranking_classifiers/es_xx.json")))
            .clone(),
        Lang::PT => PT_XX
            .get_or_init(|| load(include_str!("../ranking_classifiers/pt_xx.json")))
            .clone(),
        Lang::TR => TR_XX
            .get_or_init(|| load(include_str!("../ranking_classifiers/tr_xx.json")))
            .clone(),
        _ => HashMap::new(),
    }
}

pub fn rank_nodes(nodes: Vec<Node>, locale: &Locale, dims: &[DimensionKind]) -> Vec<Node> {
    let classifiers = classifiers_for_locale(locale);
    let candidates: Vec<Candidate> = nodes
        .into_iter()
        .filter(|n| n.token_data.dimension_kind().is_some())
        .map(|node| {
            let dim = node.token_data.dimension_kind();
            Candidate {
                score: score_node(&classifiers, &node),
                target: dims.is_empty() || dim.map(|d| dims.contains(&d)).unwrap_or(false),
                node,
            }
        })
        .collect();

    let winners: Vec<Node> = candidates
        .iter()
        .filter(|x| {
            !candidates
                .iter()
                .any(|y| compare_candidate(x, y) == Ordering::Less)
        })
        .map(|c| c.node.clone())
        .collect();

    // Dedup matching Haskell's Set.fromList on ResolvedToken, which uses
    // (range, rval, latent) — notably excluding the rule/node.
    // Since we operate on unresolved Nodes, we use token_data debug string
    // as a proxy for the resolved value.
    let mut uniq = Vec::new();
    let mut seen = HashSet::new();
    for n in winners {
        let key = (n.range.start, n.range.end, format!("{:?}", n.token_data));
        if seen.insert(key) {
            uniq.push(n);
        }
    }

    uniq.sort_by(|a, b| {
        a.range
            .start
            .cmp(&b.range.start)
            .then_with(|| a.range.end.cmp(&b.range.end))
    });
    uniq
}

/// Remove overlapping entities, keeping the longest/first.
pub fn remove_overlapping(entities: Vec<Entity>) -> Vec<Entity> {
    if entities.is_empty() {
        return entities;
    }

    let mut result: Vec<Entity> = Vec::new();

    for entity in entities {
        let dominated = result.iter().any(|existing| {
            existing.start <= entity.start
                && entity.end <= existing.end
                && (existing.start < entity.start || entity.end < existing.end)
        });

        if !dominated {
            result.retain(|existing| {
                !(entity.start <= existing.start
                    && existing.end <= entity.end
                    && (entity.start < existing.start || existing.end < entity.end))
            });
            result.push(entity);
        }
    }

    result
}
