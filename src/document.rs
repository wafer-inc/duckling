/// A Document wraps the input text and provides efficient lookup
/// for word boundaries, adjacency checks, etc.
/// Ported from Haskell Duckling's Document.hs
#[derive(Debug, Clone)]
pub struct Document {
    text: String,
    /// first_non_adjacent[i] = index of first non-whitespace byte at or after position i.
    /// If none exists, equals byte_len. Used for O(1) adjacency checks.
    first_non_adjacent: Vec<usize>,
}

impl Document {
    pub fn new(text: &str) -> Self {
        let byte_len = text.len();

        // Precompute first_non_adjacent: for each byte position, the index of the
        // first non-whitespace character at or after that position.
        let mut first_non_adjacent = vec![byte_len; byte_len.saturating_add(1)];
        let mut next_non_ws = byte_len;
        for (byte_pos, ch) in text.char_indices().rev() {
            if !ch.is_whitespace() {
                next_non_ws = byte_pos;
            }
            // Fill all byte positions within this character
            for item in first_non_adjacent
                .iter_mut()
                .skip(byte_pos)
                .take(ch.len_utf8())
            {
                *item = next_non_ws;
            }
        }

        Document {
            text: text.to_string(),
            first_non_adjacent,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// Check if two ranges are adjacent (no non-whitespace between them).
    /// Uses precomputed first_non_adjacent array for O(1) lookup.
    pub fn is_adjacent(&self, end_a: usize, start_b: usize) -> bool {
        if end_a > start_b {
            return false;
        }
        self.first_non_adjacent[end_a] >= start_b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_document() {
        let doc = Document::new("hello world");
        assert_eq!(doc.text(), "hello world");
    }

    #[test]
    fn test_adjacency() {
        let doc = Document::new("hello world");
        assert!(doc.is_adjacent(5, 6)); // space between
        assert!(doc.is_adjacent(5, 5)); // same pos is adjacent (empty range)
        assert!(doc.is_adjacent(0, 0));
    }
}
