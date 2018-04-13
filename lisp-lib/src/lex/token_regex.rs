use regex::Regex;

macro_rules! lisp_regex {
    ($exp:expr) => {
        Regex::new($exp).unwrap()
    };
}

lazy_static! {
    static ref VAR_TOKEN: Regex = lisp_regex!(r"^[A-Za-z]+$");
    static ref VAL_TOKEN: Regex = lisp_regex!(r"^[0-9]+$");
}

// Define strings that should form operators.
static OPS: [&str; 4] = [
    "+",
    "-",
    "*",
    "/",
    // "=", "+=", "-=", "*=", "/=", // FIXUP: Add assignment operators.
];

/// Returns true/false based on whether a token is a valid symbol.
pub fn is_variable_token(token: &str) -> bool {
    VAR_TOKEN.is_match(token)
}

/// Returns true/false based on whether a token is a valid value.
pub fn is_value_token(token: &str) -> bool {
    VAL_TOKEN.is_match(token)
}

/// Returns true/false based on whether a token is an operation character.
pub fn is_op_token(token: &str) -> bool {
    OPS.contains(&token.as_ref())
}

/// Returns true/false based on whether a token is a parenthetical.
pub fn is_paren(c: char) -> bool {
    c == '(' || c == ')'
}
