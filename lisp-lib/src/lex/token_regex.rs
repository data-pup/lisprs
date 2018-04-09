use regex::Regex;

macro_rules! lisp_regex { ($exp:expr) => { Regex::new($exp).unwrap() } }

lazy_static! {
    static ref VAR_TOKEN:  Regex = lisp_regex!(r"^[A-Za-z]+$");
    static ref VAL_TOKEN:  Regex = lisp_regex!(r"^[0-9]+$");
}

// Define strings that should form operators.
static OPS: [&str; 4] = [
    "+", "-", "*", "/",
    // "=", "+=", "-=", "*=", "/=", // FIXUP: Add assignment operators.
];

/// Returns true/false based on whether a token is a valid symbol.
pub fn is_variable_token(token: &str) -> bool { VAR_TOKEN.is_match(token) }

/// Returns true/false based on whether a token is a valid value.
pub fn is_value_token(token: &str) -> bool { VAL_TOKEN.is_match(token) }

/// Returns true/false based on whether a token is an operation character.
pub fn is_op_token(token: &str) -> bool { OPS.contains(&token.as_ref()) }

/// Returns true/false based on whether a token is a parenthetical.
pub fn is_paren(c: char) -> bool { c == '(' || c == ')' }

/// Returns true/false based on whether a char is syntactically relevant.
pub fn is_syntax_char(c: char) -> bool { is_op_token(&c.to_string()) || is_paren(c) }

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
        for curr in ["+", "-", "*", "/",].into_iter() {
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
    }
}
