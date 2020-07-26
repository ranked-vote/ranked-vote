use regex::Regex;

pub fn normalize_name(name: &str) -> String {
    // Convert "Last, First" names into "First Last"
    let fix_comma = Regex::new("(?P<last>.+), (?P<first>.+)").unwrap();
    let fixed = fix_comma.replace(name, "$first $last").to_string();

    let chars: Vec<char> = fixed.chars().collect();
    let mut new_chars: Vec<char> = Vec::with_capacity(chars.len());

    let mut first = true;
    let mut in_quote = false;
    for ch in chars {
        if ch == ' ' || ch == '-' {
            first = true;
            new_chars.push(ch);
        } else if ch == '"' && !in_quote {
            new_chars.push('“');
            in_quote = true;
            first = true;
        } else if ch == '"' {
            new_chars.push('”');
            in_quote = false;
        } else if first {
            new_chars.push(ch);
            first = false;
        } else {
            new_chars.extend(ch.to_lowercase());
        }
    }

    new_chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_case() {
        assert_eq!("Bob Vance", normalize_name("BOB VANCE"));
        assert_eq!("Bob Vance", normalize_name("Bob Vance"));
    }

    #[test]
    fn test_quote() {
        assert_eq!(
            "Joe “The Dog” Johnson",
            normalize_name("Joe \"THE DOG\" JOHNSON")
        );
    }

    #[test]
    fn test_unicode() {
        assert_eq!("José Peters", normalize_name("JOSÉ PETERS"))
    }

    #[test]
    fn test_comma() {
        assert_eq!("Bob Vance", normalize_name("VANCE, BOB"))
    }
}
