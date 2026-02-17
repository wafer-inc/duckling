use crate::types::Node;
use std::collections::BTreeMap;

/// A Stash stores parsed nodes keyed by their start position.
/// This allows efficient lookup of nodes at a given position.
#[derive(Debug, Clone, Default)]
pub struct Stash {
    nodes: BTreeMap<usize, Vec<Node>>,
    count: usize,
}

impl Stash {
    pub fn new() -> Self {
        Stash {
            nodes: BTreeMap::new(),
            count: 0,
        }
    }

    pub fn add(&mut self, node: Node) {
        self.nodes.entry(node.range.start).or_default().push(node);
        self.count = self.count.saturating_add(1);
    }

    pub fn all_nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values().flat_map(|v| v.iter())
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn merge(&mut self, other: &Stash) {
        for node in other.all_nodes() {
            self.add(node.clone());
        }
    }

    /// Iterate over nodes starting at or after the given position.
    /// Uses BTreeMap's range for efficient lookup.
    pub fn nodes_starting_from(&self, pos: usize) -> impl Iterator<Item = &Node> {
        self.nodes.range(pos..).flat_map(|(_, v)| v.iter())
    }
}
