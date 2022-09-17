use lazy_static::lazy_static;
use regex::Regex;

pub fn normalize_name(name: &str, flip_comma: bool) -> String {
    let mut fixed = if flip_comma {
        // Convert "Last, First" names into "First Last"
        lazy_static! {
            static ref FIX_COMMA: Regex = Regex::new("(?P<last>.+), (?P<first>.+)").unwrap();
        }

        FIX_COMMA.replace(name, "$first $last").to_string()
    } else {
        name.to_string()
    };

    if fixed.split("''").count() > 2 {
        fixed = fixed.replace("''", "\"");
    } else {
        fixed = fixed.replace("''", "\'");
    }

    let chars: Vec<char> = fixed.chars().collect();
    let mut new_chars: Vec<char> = Vec::with_capacity(chars.len());

    let mut first = true;
    let mut in_quote = false;
    for ch in chars {
        if ch == ' ' || ch == '-' || ch == '.' || ch == '\'' {
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
        assert_eq!("Bob Vance", normalize_name("BOB VANCE", false));
        assert_eq!("Bob Vance", normalize_name("Bob Vance", false));
    }

    #[test]
    fn test_quote() {
        assert_eq!(
            "Joe “The Dog” Johnson",
            normalize_name("Joe \"THE DOG\" JOHNSON", false)
        );
    }

    #[test]
    fn test_unicode() {
        assert_eq!("José Peters", normalize_name("JOSÉ PETERS", false));
    }

    #[test]
    fn test_comma() {
        assert_eq!("Bob Vance", normalize_name("VANCE, BOB", true));
        assert_eq!("Jim Jones Jr.", normalize_name("JIM JONES JR.", false));
    }

    #[test]
    fn test_dot() {
        assert_eq!("Joe A.B. John", normalize_name("JOE A.B. JOHN", false));
    }

    #[test]
    fn test_apostrophe() {
        assert_eq!("Joe O'Brian", normalize_name("JOE O''BRIAN", false));
        assert_eq!("Joe O'Brian", normalize_name("JOE O'BRIAN", false));
    }
}
