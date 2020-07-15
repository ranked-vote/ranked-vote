pub struct UnicodeString {
    chars: Vec<char>,
}

impl UnicodeString {
    pub fn new(string: &str) -> UnicodeString {
        UnicodeString {
            chars: string.chars().collect(),
        }
    }

    pub fn slice(&self, range: std::ops::Range<usize>) -> String {
        self.chars[range].iter().collect()
    }
}
