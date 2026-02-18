// Port of Duckling/Ranking/Train.hs
// Naive Bayes classifier training with Laplace smoothing.

use std::collections::{HashMap, HashSet};

use crate::engine;
use crate::ranking::{extract_features, BagOfFeatures, ClassData, Classifier, Classifiers};
use crate::resolve::{Context, Options};
use crate::types::{DimensionKind, Entity, Node, Rule};

type Datum = (BagOfFeatures, bool);
type Dataset = HashMap<String, Vec<Datum>>;

// Key for deduplicating nodes (matches engine::dedup_key)
type NodeKey = (usize, usize, Option<String>, String);

fn node_key(node: &Node) -> NodeKey {
    (
        node.range.start,
        node.range.end,
        node.rule_name.clone(),
        format!("{:?}", node.token_data),
    )
}

/// Port of Haskell's `subnodes`. Recursively collects all non-leaf nodes
/// (nodes with children) including the root.
fn collect_subnodes(node: &Node) -> HashSet<NodeKey> {
    if node.children.is_empty() {
        return HashSet::new();
    }
    let mut result = HashSet::new();
    result.insert(node_key(node));
    for child in &node.children {
        for key in collect_subnodes(child) {
            result.insert(key);
        }
    }
    result
}

/// Collect all nodes by key with a reference to the node itself for feature extraction.
fn collect_subnodes_with_nodes(node: &Node) -> Vec<(NodeKey, Node)> {
    if node.children.is_empty() {
        return Vec::new();
    }
    let mut result = vec![(node_key(node), node.clone())];
    for child in &node.children {
        result.extend(collect_subnodes_with_nodes(child));
    }
    result
}

/// Port of Haskell's `makeDataset1`. Augment the dataset with one example.
fn make_dataset1(
    rules: &[Rule],
    context: &Context,
    options: &Options,
    dataset: &mut Dataset,
    sentence: &str,
    predicate: &dyn Fn(&Entity) -> bool,
    dims: &[DimensionKind],
) {
    let tokens = engine::parse_and_resolve_with_nodes(sentence, rules, context, options, dims);

    let (ok, ko): (Vec<_>, Vec<_>) = tokens
        .into_iter()
        .partition(|(_, entity)| predicate(entity));

    // Collect subnode keys from ok tokens
    let nodes_ok_keys: HashSet<NodeKey> = ok
        .iter()
        .flat_map(|(node, _)| collect_subnodes(node))
        .collect();

    // Collect subnodes from ko tokens, minus those in nodes_ok
    let nodes_ko_keys: HashSet<NodeKey> = ko
        .iter()
        .flat_map(|(node, _)| collect_subnodes(node))
        .filter(|key| !nodes_ok_keys.contains(key))
        .collect();

    // Collect actual nodes for feature extraction from ok
    let mut ok_nodes: HashMap<NodeKey, Node> = HashMap::new();
    for (node, _) in &ok {
        for (key, n) in collect_subnodes_with_nodes(node) {
            if nodes_ok_keys.contains(&key) {
                ok_nodes.entry(key).or_insert(n);
            }
        }
    }

    // Collect actual nodes for feature extraction from ko
    let mut ko_nodes: HashMap<NodeKey, Node> = HashMap::new();
    for (node, _) in &ko {
        for (key, n) in collect_subnodes_with_nodes(node) {
            if nodes_ko_keys.contains(&key) {
                ko_nodes.entry(key).or_insert(n);
            }
        }
    }

    // Update dataset with OK nodes
    for node in ok_nodes.values() {
        if let Some(rule) = &node.rule_name {
            let feats = extract_features(node);
            dataset.entry(rule.clone()).or_default().push((feats, true));
        }
    }

    // Update dataset with KO nodes
    for node in ko_nodes.values() {
        if let Some(rule) = &node.rule_name {
            let feats = extract_features(node);
            dataset
                .entry(rule.clone())
                .or_default()
                .push((feats, false));
        }
    }
}

/// Port of Haskell's `makeClass`. Compute prior and likelihood log-probabilities for one class.
fn make_class(
    feats: &HashMap<String, i32>,
    total: i32,
    class_total: i32,
    voc_size: i32,
) -> ClassData {
    let prior = (class_total as f64 / total as f64).ln();
    let feat_sum: i32 = feats.values().sum();
    let denum = voc_size.saturating_add(feat_sum);
    let unseen = (1.0_f64 / (denum as f64 + 1.0)).ln();
    let likelihoods: HashMap<String, f64> = feats
        .iter()
        .map(|(f, &count)| {
            let ll = ((count as f64 + 1.0) / denum as f64).ln();
            (f.clone(), ll)
        })
        .collect();
    ClassData {
        prior,
        unseen,
        likelihoods,
        n: class_total,
    }
}

/// Port of Haskell's `train`. Train a classifier for a single rule.
fn train(datums: &[Datum]) -> Classifier {
    let total = datums.len() as i32;
    let (ok, ko): (Vec<_>, Vec<_>) = datums.iter().partition(|(_, class)| *class);

    // Merge feature counts per class
    let mut ok_counts: HashMap<String, i32> = HashMap::new();
    for (feats, _) in &ok {
        for (f, &count) in feats {
            let entry = ok_counts.entry(f.clone()).or_insert(0);
            *entry = entry.saturating_add(count);
        }
    }

    let mut ko_counts: HashMap<String, i32> = HashMap::new();
    for (feats, _) in &ko {
        for (f, &count) in feats {
            let entry = ko_counts.entry(f.clone()).or_insert(0);
            *entry = entry.saturating_add(count);
        }
    }

    // Vocabulary size = union of features from both classes
    let mut all_feats: HashSet<&String> = ok_counts.keys().collect();
    all_feats.extend(ko_counts.keys());
    let voc_size = all_feats.len() as i32;

    let ok_data = make_class(&ok_counts, total, ok.len() as i32, voc_size);
    let ko_data = make_class(&ko_counts, total, ko.len() as i32, voc_size);

    Classifier { ok_data, ko_data }
}

/// Corpus type for training. Matches Haskell's `(Context, Options, [Example])`.
pub struct TrainingCorpus {
    /// The reference context (time, locale) for resolving examples.
    pub context: Context,
    /// Parsing options.
    pub options: Options,
    /// Each example is `(text, predicate)` where the predicate checks the resolved entity.
    #[allow(clippy::type_complexity)]
    pub examples: Vec<(String, Box<dyn Fn(&Entity) -> bool>)>,
}

/// Port of Haskell's `makeDataset`. Build a dataset from rules and corpus.
fn make_dataset(rules: &[Rule], corpus: &TrainingCorpus, dims: &[DimensionKind]) -> Dataset {
    let mut dataset = Dataset::new();
    for (sentence, predicate) in &corpus.examples {
        make_dataset1(
            rules,
            &corpus.context,
            &corpus.options,
            &mut dataset,
            sentence,
            predicate.as_ref(),
            dims,
        );
    }
    dataset
}

/// Port of Haskell's `makeClassifiers`. Train classifiers from rules and corpus.
pub fn make_classifiers(
    rules: &[Rule],
    corpus: &TrainingCorpus,
    dims: &[DimensionKind],
) -> Classifiers {
    let dataset = make_dataset(rules, corpus, dims);
    dataset
        .iter()
        .map(|(rule, datums)| (rule.clone(), train(datums)))
        .collect()
}
