#[cfg(test)]
mod char_regex_tests {
    use lex::token_regex;

    #[test]
    fn regexes_should_identify_parens() {
        for curr in ['(', ')'].into_iter() {
            let is_match = token_regex::is_paren(*curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ['a', '[', '1'].into_iter() {
            let is_match = token_regex::is_paren(*curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }

    #[test]
    fn regexes_should_match_operators() {
        for curr in ["+", "-", "*", "/"].into_iter() {
            let is_match = token_regex::is_op_token(&curr.to_string());
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ["!", ",", ".", "?"].into_iter() {
            let is_match = token_regex::is_op_token(&curr.to_string());
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }
}

#[cfg(test)]
mod token_regex_tests {
    use lex::token_regex;

    #[test]
    fn regexes_should_identify_symbols() {
        for curr in ["h", "H", "hello", "world", "Hello", "WORLD"].into_iter() {
            let is_match = token_regex::is_variable_token(&curr.to_string());
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ["1", "10", "+", "_", "(", "(hello", "hello)"].into_iter() {
            let is_match = token_regex::is_variable_token(&curr.to_string());
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }

    #[test]
    fn regexes_should_identify_values() {
        for curr in ["1", "10", "999"].into_iter() {
            let is_match = token_regex::is_value_token(&curr.to_string());
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ["+", "variable", "("].into_iter() {
            let is_match = token_regex::is_value_token(&curr.to_string());
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }
}
