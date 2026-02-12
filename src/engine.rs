use std::collections::HashSet;

use regex::Regex;

use crate::document::Document;
use crate::resolve::{Context, Options};
use crate::stash::Stash;
use crate::types::{
    DimensionKind, Entity, Node, PatternItem, Range, RegexMatchData, Rule, TokenData,
};

type RegexMatches = Vec<(Range, Vec<Option<String>>)>;

/// Parse text and resolve all entities.
pub fn parse_and_resolve(
    text: &str,
    rules: &[Rule],
    context: &Context,
    options: &Options,
    dims: &[DimensionKind],
) -> Vec<Entity> {
    let stash = parse_string(text, rules);
    let doc_text = text;

    let mut entities: Vec<Entity> = Vec::new();
    for node in stash.all_nodes() {
        if let Some(dk) = node.token_data.dimension_kind() {
            if !dims.is_empty() && !dims.contains(&dk) {
                continue;
            }
        } else {
            continue;
        }
        if let Some(entity) = crate::resolve::resolve(node, context, options, doc_text) {
            entities.push(entity);
        }
    }

    entities
}

/// Run the saturation-based parsing loop.
pub fn parse_string(text: &str, rules: &[Rule]) -> Stash {
    let doc = Document::new(text);
    let mut stash = Stash::new();

    // Pre-compute regex matches for all regex-leading rules once.
    // Regex matches against document text never change between iterations.
    let regex_cache: Vec<Option<RegexMatches>> = rules
        .iter()
        .map(|rule| {
            if rule.pattern.is_empty() {
                return None;
            }
            if let PatternItem::Regex(ref re) = rule.pattern[0] {
                Some(find_regex_matches(&doc, re))
            } else {
                None
            }
        })
        .collect();

    // Track seen (start, end, rule_name) to deduplicate and prevent exponential growth
    let mut seen: HashSet<(usize, usize, Option<String>)> = HashSet::new();

    // Phase 1: Apply all regex-leading rules to find initial tokens
    let initial = apply_regex_rules(rules, &regex_cache);
    for node in initial.all_nodes() {
        let key = (node.range.start, node.range.end, node.rule_name.clone());
        seen.insert(key);
    }
    stash.merge(&initial);

    // Phase 2: Saturation loop - keep applying rules until no new tokens
    let max_iterations = 10;
    for _ in 0..max_iterations {
        let new_stash = apply_all_rules(&doc, rules, &stash, &regex_cache);
        let mut actually_new = Stash::new();
        for node in new_stash.all_nodes() {
            let key = (node.range.start, node.range.end, node.rule_name.clone());
            if seen.insert(key) {
                actually_new.add(node.clone());
            }
        }
        if actually_new.is_empty() {
            break;
        }
        stash.merge(&actually_new);
    }

    stash
}

/// Apply regex-leading rules to the document to find initial tokens.
/// Uses pre-computed regex cache.
fn apply_regex_rules(
    rules: &[Rule],
    regex_cache: &[Option<RegexMatches>],
) -> Stash {
    let mut stash = Stash::new();

    for (i, rule) in rules.iter().enumerate() {
        if rule.pattern.is_empty() {
            continue;
        }
        if let PatternItem::Regex(_) = &rule.pattern[0] {
            let matches = match &regex_cache[i] {
                Some(m) => m,
                None => continue,
            };
            for (range, groups) in matches {
                let regex_node = Node {
                    range: *range,
                    token_data: TokenData::RegexMatch(RegexMatchData {
                        groups: groups.clone(),
                    }),
                    children: Vec::new(),
                    rule_name: None,
                };

                if rule.pattern.len() == 1 {
                    // Single-pattern rule: produce directly
                    if let Some(token_data) = (rule.production)(&[&regex_node]) {
                        let mut node = Node::new(*range, token_data);
                        node.rule_name = Some(rule.name.clone());
                        node.children = vec![regex_node];
                        stash.add(node);
                    }
                } else {
                    // Multi-pattern rule: store the regex match for later combination
                    stash.add(regex_node);
                }
            }
        }
    }

    stash
}

/// Apply all rules against the current stash to find new tokens.
/// Skips single-pattern regex rules (already fully handled in phase 1).
fn apply_all_rules(
    doc: &Document,
    rules: &[Rule],
    stash: &Stash,
    regex_cache: &[Option<RegexMatches>],
) -> Stash {
    let mut new_stash = Stash::new();

    for (i, rule) in rules.iter().enumerate() {
        // Skip single-pattern regex rules - already fully handled in phase 1
        if rule.pattern.len() == 1 {
            if let PatternItem::Regex(_) = &rule.pattern[0] {
                continue;
            }
        }
        let cached = regex_cache[i].as_ref();
        let produced = match_rule(doc, rule, stash, cached);
        for node in produced {
            new_stash.add(node);
        }
    }

    new_stash
}

/// Try to match a rule against the document and stash, producing new nodes.
/// Uses cached regex matches when available.
fn match_rule(
    doc: &Document,
    rule: &Rule,
    stash: &Stash,
    cached_regex: Option<&RegexMatches>,
) -> Vec<Node> {
    let mut results = Vec::new();

    if rule.pattern.is_empty() {
        return results;
    }

    match &rule.pattern[0] {
        PatternItem::Regex(_) => {
            // Use cached regex matches
            let matches = match cached_regex {
                Some(m) => m,
                None => return results,
            };
            for (range, groups) in matches {
                let regex_node = Node {
                    range: *range,
                    token_data: TokenData::RegexMatch(RegexMatchData {
                        groups: groups.clone(),
                    }),
                    children: Vec::new(),
                    rule_name: None,
                };

                if rule.pattern.len() == 1 {
                    if let Some(token_data) = (rule.production)(&[&regex_node]) {
                        let mut node = Node::new(*range, token_data);
                        node.rule_name = Some(rule.name.clone());
                        node.children = vec![regex_node];
                        results.push(node);
                    }
                } else {
                    // Continue matching remaining patterns
                    let continuations =
                        match_remaining(doc, rule, stash, 1, range.end, vec![regex_node]);
                    results.extend(continuations);
                }
            }
        }
        PatternItem::Dimension(dim) => {
            // Start with dimension matches from the stash
            for node in stash.all_nodes() {
                if node.token_data.dimension_kind() == Some(*dim) {
                    if rule.pattern.len() == 1 {
                        if let Some(token_data) = (rule.production)(&[node]) {
                            let mut new_node = Node::new(node.range, token_data);
                            new_node.rule_name = Some(rule.name.clone());
                            new_node.children = vec![node.clone()];
                            results.push(new_node);
                        }
                    } else {
                        let continuations = match_remaining(
                            doc,
                            rule,
                            stash,
                            1,
                            node.range.end,
                            vec![node.clone()],
                        );
                        results.extend(continuations);
                    }
                }
            }
        }
        PatternItem::Predicate(pred) => {
            for node in stash.all_nodes() {
                if pred(&node.token_data) {
                    if rule.pattern.len() == 1 {
                        if let Some(token_data) = (rule.production)(&[node]) {
                            let mut new_node = Node::new(node.range, token_data);
                            new_node.rule_name = Some(rule.name.clone());
                            new_node.children = vec![node.clone()];
                            results.push(new_node);
                        }
                    } else {
                        let continuations = match_remaining(
                            doc,
                            rule,
                            stash,
                            1,
                            node.range.end,
                            vec![node.clone()],
                        );
                        results.extend(continuations);
                    }
                }
            }
        }
    }

    results
}

/// Continue matching the remaining pattern items after a partial match.
/// Uses position-filtered stash iteration for efficiency.
fn match_remaining(
    doc: &Document,
    rule: &Rule,
    stash: &Stash,
    pattern_idx: usize,
    after_pos: usize,
    matched_so_far: Vec<Node>,
) -> Vec<Node> {
    let mut results = Vec::new();

    if pattern_idx >= rule.pattern.len() {
        // All patterns matched - produce the result
        let refs: Vec<&Node> = matched_so_far.iter().collect();
        if let Some(token_data) = (rule.production)(&refs) {
            let start = matched_so_far.first().unwrap().range.start;
            let end = matched_so_far.last().unwrap().range.end;
            let mut node = Node::new(Range::new(start, end), token_data);
            node.rule_name = Some(rule.name.clone());
            node.children = matched_so_far;
            results.push(node);
        }
        return results;
    }

    match &rule.pattern[pattern_idx] {
        PatternItem::Regex(re) => {
            // Try to match regex starting near after_pos
            let text = doc.lower();
            let original = doc.text();
            if after_pos <= text.len() {
                let search_text = &text[after_pos..];
                if let Some(m) = re.find(search_text) {
                    let abs_start = after_pos + m.start();
                    let abs_end = after_pos + m.end();
                    // Must be adjacent
                    if doc.is_adjacent(after_pos, abs_start) {
                        let caps = re.captures(search_text);
                        let groups = match &caps {
                            Some(c) => {
                                let mut g = Vec::new();
                                for i in 0..c.len() {
                                    g.push(c.get(i).map(|m2| {
                                        let s = after_pos + m2.start();
                                        let e = after_pos + m2.end();
                                        original[s..e].to_string()
                                    }));
                                }
                                g
                            }
                            None => Vec::new(),
                        };
                        let regex_node = Node {
                            range: Range::new(abs_start, abs_end),
                            token_data: TokenData::RegexMatch(RegexMatchData { groups }),
                            children: Vec::new(),
                            rule_name: None,
                        };
                        let mut next_matched = matched_so_far.clone();
                        next_matched.push(regex_node);
                        let cont = match_remaining(
                            doc,
                            rule,
                            stash,
                            pattern_idx + 1,
                            abs_end,
                            next_matched,
                        );
                        results.extend(cont);
                    }
                }
            }
        }
        PatternItem::Dimension(dim) => {
            // Use position-filtered iteration instead of scanning all nodes
            for node in stash.nodes_starting_from(after_pos) {
                if node.token_data.dimension_kind() == Some(*dim)
                    && doc.is_adjacent(after_pos, node.range.start)
                {
                    let mut next_matched = matched_so_far.clone();
                    next_matched.push(node.clone());
                    let cont = match_remaining(
                        doc,
                        rule,
                        stash,
                        pattern_idx + 1,
                        node.range.end,
                        next_matched,
                    );
                    results.extend(cont);
                }
            }
        }
        PatternItem::Predicate(pred) => {
            // Use position-filtered iteration instead of scanning all nodes
            for node in stash.nodes_starting_from(after_pos) {
                if pred(&node.token_data) && doc.is_adjacent(after_pos, node.range.start) {
                    let mut next_matched = matched_so_far.clone();
                    next_matched.push(node.clone());
                    let cont = match_remaining(
                        doc,
                        rule,
                        stash,
                        pattern_idx + 1,
                        node.range.end,
                        next_matched,
                    );
                    results.extend(cont);
                }
            }
        }
    }

    results
}

/// Find all regex matches in the document.
/// Matches against the lowered text but extracts captured groups from the
/// original text to preserve case (important for email, URL, etc.).
fn find_regex_matches(doc: &Document, re: &Regex) -> Vec<(Range, Vec<Option<String>>)> {
    let lower = doc.lower();
    let original = doc.text();
    let mut matches = Vec::new();

    for caps in re.captures_iter(lower) {
        let m = caps.get(0).unwrap();
        let range = Range::new(m.start(), m.end());
        let groups = extract_groups(&caps, original);
        matches.push((range, groups));
    }

    matches
}

fn extract_groups(caps: &regex::Captures, original: &str) -> Vec<Option<String>> {
    let mut groups = Vec::new();
    for i in 0..caps.len() {
        groups.push(caps.get(i).map(|m| {
            // Extract text from original (preserves case) using byte offsets
            original[m.start()..m.end()].to_string()
        }));
    }
    groups
}
