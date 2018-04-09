use regex::Regex;

lazy_static! {
    static ref VAR_TOKEN:  Regex = Regex::new(r"^[A-Za-z]+$").unwrap();
    static ref VAL_TOKEN:  Regex = Regex::new(r"^[0-9]+$").unwrap();
}

static OP_CHARS: [char; 4] = ['+', '-', '*', '/'];

/// Returns true/false based on whether a token is a valid symbol.
pub fn is_variable_token(token: &String) -> bool { VAR_TOKEN.is_match(token) }

/// Returns true/false based on whether a token is a valid value.
pub fn is_value_token(token: &String) -> bool { VAL_TOKEN.is_match(token) }

/// Returns true/false based on whether a token is a parenthetical.
pub fn is_paren(c: char) -> bool { c == '(' || c == ')' }

/// Returns true/false based on whether a token is an operation character.
pub fn is_op_char(c: char) -> bool { OP_CHARS.contains(&c) }

/// Returns true/false based on whether a char is syntactically relevant.
pub fn is_syntax_char(c: char) -> bool { is_op_char(c) || is_paren(c) }

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
        for curr in ['+', '-', '*', '/',].into_iter() {
            let is_match = token_regex::is_op_char(*curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ['!', ',', '.', '?'].into_iter() {
            let is_match = token_regex::is_op_char(*curr);
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
    }
}
