use regex::Regex;

lazy_static! {
    static ref VAR_TOKEN:  Regex = Regex::new(r"[A-Za-z]").unwrap();
    static ref PAREN_CHAR: Regex = Regex::new(r"^[()]$").unwrap();
    static ref OP_CHAR:    Regex = Regex::new(r"^[+\-*/]$").unwrap();
}

/// Split a raw token into its syntactically meaningful components.
pub fn process_raw_token(token: String) -> Vec<String> {
    vec![token]
}

#[cfg(test)]
mod token_tests {
    use lex::process_token;
    use regex;

    #[test]
    fn regexes_should_identify_parens() {
        for curr in ["(", ")"].into_iter() {
            let is_match = process_token::PAREN_CHAR.is_match(curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ["a", "[", "1"].into_iter() {
            let is_match = process_token::PAREN_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }

        for curr in ["(+", "hello)"].into_iter() {
            let is_match = process_token::PAREN_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }

    #[test]
    fn regexes_should_match_operators() {
        for curr in ["+", "-", "*", "/",].into_iter() {
            let is_match = process_token::OP_CHAR.is_match(curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ["!", ",", ".", "?"].into_iter() {
            let is_match = process_token::OP_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }

        for curr in ["++", "/hello", "hello/"].into_iter() {
            let is_match = process_token::OP_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }
}
