/// A Document wraps the input text and provides efficient lookup
/// for word boundaries, adjacency checks, etc.
/// Ported from Haskell Duckling's Document.hs
#[derive(Debug, Clone)]
pub struct Document {
    text: String,
    lower: String,
    byte_len: usize,
    /// first_non_adjacent[i] = index of first non-whitespace byte at or after position i.
    /// If none exists, equals byte_len. Used for O(1) adjacency checks.
    first_non_adjacent: Vec<usize>,
}

impl Document {
    pub fn new(text: &str) -> Self {
        let lower = text.to_lowercase();
        let byte_len = text.len();

        // Precompute first_non_adjacent: for each byte position, the index of the
        // first non-whitespace character at or after that position.
        let mut first_non_adjacent = vec![byte_len; byte_len + 1];
        let mut next_non_ws = byte_len;
        for (byte_pos, ch) in text.char_indices().rev() {
            if !ch.is_whitespace() {
                next_non_ws = byte_pos;
            }
            // Fill all byte positions within this character
            for j in byte_pos..byte_pos + ch.len_utf8() {
                first_non_adjacent[j] = next_non_ws;
            }
        }

        Document {
            text: text.to_string(),
            lower,
            byte_len,
            first_non_adjacent,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn lower(&self) -> &str {
        &self.lower
    }

    pub fn len(&self) -> usize {
        self.byte_len
    }

    pub fn is_empty(&self) -> bool {
        self.byte_len == 0
    }

    /// Check if two ranges are adjacent (no non-whitespace between them).
    /// Uses precomputed first_non_adjacent array for O(1) lookup.
    pub fn is_adjacent(&self, end_a: usize, start_b: usize) -> bool {
        if end_a > start_b {
            return false;
        }
        self.first_non_adjacent[end_a] >= start_b
    }

    /// Check if position is at a word boundary
    pub fn is_word_boundary(&self, pos: usize) -> bool {
        if pos == 0 || pos >= self.byte_len {
            return true;
        }
        let bytes = self.text.as_bytes();
        if pos < self.byte_len {
            let before = bytes.get(pos.wrapping_sub(1)).copied();
            let after = bytes.get(pos).copied();
            match (before, after) {
                (Some(b), Some(a)) => {
                    let b_alnum = (b as char).is_alphanumeric();
                    let a_alnum = (a as char).is_alphanumeric();
                    b_alnum != a_alnum
                }
                _ => true,
            }
        } else {
            true
        }
    }

    /// Check if a range represents a valid token boundary
    pub fn is_range_valid(&self, start: usize, end: usize) -> bool {
        start <= end && end <= self.byte_len
    }

    /// Get the substring for a range
    pub fn substring(&self, start: usize, end: usize) -> &str {
        &self.text[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_document() {
        let doc = Document::new("hello world");
        assert_eq!(doc.text(), "hello world");
        assert_eq!(doc.lower(), "hello world");
        assert_eq!(doc.len(), 11);
    }

    #[test]
    fn test_adjacency() {
        let doc = Document::new("hello world");
        assert!(doc.is_adjacent(5, 6)); // space between
        assert!(doc.is_adjacent(5, 5)); // same pos is adjacent (empty range)
        assert!(doc.is_adjacent(0, 0));
    }

    #[test]
    fn test_word_boundary() {
        let doc = Document::new("hello world");
        assert!(doc.is_word_boundary(0));
        assert!(doc.is_word_boundary(5));
        assert!(doc.is_word_boundary(6));
        assert!(!doc.is_word_boundary(3));
    }
}
