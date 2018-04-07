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
mod raw_token_processing_tests {
    use lex::process_token;

    #[test]
    fn basic_tokens_should_be_returned_identically() {
        for input in ["hello", "+", "(", "0"].into_iter() {
            let input_s = input.to_string();
            let expected = vec![input_s.clone()];
            let result = process_token::process_raw_token(input_s.clone());
            assert_eq!(result, expected, "Incorrectly split: {}", input_s);
        }
    }

    #[test]
    fn token_with_paren_and_op_should_be_split() {
        let input = String::from("(+");
        let result = process_token::process_raw_token(input);
        let expected = vec![String::from("("), String::from("+")];
        assert_eq!(result, expected, "Did not split parentheses with operator.");
    }
}

#[cfg(test)]
mod regex_tests {
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
