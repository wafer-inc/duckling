/// A Document wraps the input text and provides efficient lookup
/// for word boundaries, adjacency checks, etc.
/// Ported from Haskell Duckling's Document.hs
#[derive(Debug, Clone)]
pub struct Document {
    text: String,
    lower: String,
    byte_len: usize,
}

impl Document {
    pub fn new(text: &str) -> Self {
        let lower = text.to_lowercase();
        let byte_len = text.len();

        Document {
            text: text.to_string(),
            lower,
            byte_len,
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

    /// Check if two ranges are adjacent (no non-whitespace between them)
    pub fn is_adjacent(&self, end_a: usize, start_b: usize) -> bool {
        if end_a > start_b {
            return false;
        }
        let between = &self.text[end_a..start_b];
        between.chars().all(|c| c.is_whitespace())
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
