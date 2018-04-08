use regex::Regex;

lazy_static! {
    static ref VAR_TOKEN:  Regex = Regex::new(r"^[A-Za-z]+$").unwrap();
    static ref VAL_TOKEN:  Regex = Regex::new(r"^[0-9]+$").unwrap();
    static ref PAREN_CHAR: Regex = Regex::new(r"^[()]$").unwrap();
    static ref OP_CHAR:    Regex = Regex::new(r"^[+\-*/]$").unwrap();
}

// Helper functions to determine the contents of a token.
pub fn is_variable_token(token: &String) -> bool { VAR_TOKEN.is_match(token) }
pub fn is_value_token(token: &String)    -> bool { VAL_TOKEN.is_match(token) }
pub fn is_syntax_char(token: &String)   -> bool {
    PAREN_CHAR.is_match(token) || OP_CHAR.is_match(token)
}

#[cfg(test)]
mod regex_tests {
    use lex::token_regex;

    #[test]
    fn regexes_should_identify_parens() {
        for curr in ["(", ")"].into_iter() {
            let is_match = token_regex::PAREN_CHAR.is_match(curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ["a", "[", "1"].into_iter() {
            let is_match = token_regex::PAREN_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }

        for curr in ["(+", "hello)"].into_iter() {
            let is_match = token_regex::PAREN_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }

    #[test]
    fn regexes_should_match_operators() {
        for curr in ["+", "-", "*", "/",].into_iter() {
            let is_match = token_regex::OP_CHAR.is_match(curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ["!", ",", ".", "?"].into_iter() {
            let is_match = token_regex::OP_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }

        for curr in ["++", "/hello", "hello/"].into_iter() {
            let is_match = token_regex::OP_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }

    #[test]
    fn regexes_should_identify_symbols() {
        for curr in ["h", "H", "hello", "world", "Hello", "WORLD"].into_iter() {
            let is_match = token_regex::VAR_TOKEN.is_match(curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ["1", "10", "+", "_", "(", "(hello", "hello)"].into_iter() {
            let is_match = token_regex::VAR_TOKEN.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }

    #[test]
    fn regexes_should_identify_values() {
        for curr in ["1", "10", "999"].into_iter() {
            let is_match = token_regex::VAL_TOKEN.is_match(curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }
    }
}
