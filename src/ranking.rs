use crate::types::Entity;

/// Rank and filter entities using heuristic ranking.
/// Prefers longer matches and non-overlapping results.
pub fn rank(entities: &mut [Entity]) {
    // Sort by start position, then by length (longer first)
    entities.sort_by(|a, b| {
        a.start.cmp(&b.start).then_with(|| {
            let len_a = a.end.saturating_sub(a.start);
            let len_b = b.end.saturating_sub(b.start);
            len_b.cmp(&len_a) // longer first
        })
    });
}

/// Remove overlapping entities, keeping the longest/first.
pub fn remove_overlapping(entities: Vec<Entity>) -> Vec<Entity> {
    if entities.is_empty() {
        return entities;
    }

    let mut result: Vec<Entity> = Vec::new();

    for entity in entities {
        let dominated = result.iter().any(|existing| {
            // entity is strictly contained within existing (not equal span)
            existing.start <= entity.start
                && entity.end <= existing.end
                && (existing.start < entity.start || entity.end < existing.end)
        });

        if !dominated {
            // Remove any existing entities that this one strictly dominates
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
