/// A character-indexable representation of a Unicode string. This is necssary
/// because a usual Rust string can't be indexed by characters because characters
/// in UTF-8 are variable-length.
pub struct UnicodeString {
    chars: Vec<char>,
}

impl UnicodeString {
    pub fn new(string: &str) -> UnicodeString {
        UnicodeString {
            chars: string.chars().collect(),
        }
    }

    /// Slices a `UnicodeString` by the given range.
    pub fn slice(&self, range: std::ops::Range<usize>) -> String {
        self.chars[range].iter().collect()
    }
}
