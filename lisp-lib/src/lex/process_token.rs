use regex::Regex;

lazy_static! {
    static ref VAR_TOKEN:  Regex = Regex::new(r"[A-Za-z]").unwrap();
    static ref VAL_TOKEN:  Regex = Regex::new(r"[0-9]").unwrap();
    static ref PAREN_CHAR: Regex = Regex::new(r"^[()]$").unwrap();
    static ref OP_CHAR:    Regex = Regex::new(r"^[+\-*/]$").unwrap();
}

// Helper functions to determine the contents of a token.
fn is_variable(token: &String) -> bool { VAR_TOKEN.is_match(token) }
fn is_value(token: &String)    -> bool { VAL_TOKEN.is_match(token) }
fn is_syntax(token: &String)   -> bool {
    PAREN_CHAR.is_match(token) || OP_CHAR.is_match(token)
}

/// Split a raw token into its syntactically meaningful components.
pub fn process_raw_token(token: String) -> Vec<String> {
    match &token {
        s if is_variable(&s) => vec![token.clone()],
        s if is_value(&s)    => vec![token.clone()],
        s if is_syntax(&s)   => vec![token.clone()],
        _ => unimplemented!("HELLO"),
    }
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

    #[test]
    fn regexes_should_identify_symbols() {
        for curr in ["h", "H", "hello", "world", "Hello", "WORLD"].into_iter() {
            let is_match = process_token::VAR_TOKEN.is_match(curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ["1", "10", "+", "_"].into_iter() {
            let is_match = process_token::VAR_TOKEN.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }

    #[test]
    fn regexes_should_identify_values() {
        for curr in ["1", "10", "999"].into_iter() {
            let is_match = process_token::VAL_TOKEN.is_match(curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }
    }
}
