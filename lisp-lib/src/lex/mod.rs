mod process_op_token;
mod split_parens;
mod split_raw_token;
mod tests;
mod token_regex;

use lisp_token::LispToken;

pub fn get_tokens(input: &str) -> Vec<LispToken> {
    let raw_input = input.to_string();
    raw_input
        .split_whitespace()
        .map(str::to_string)
        .flat_map(split_raw_token::process_raw_lisp_token)
        .collect::<Vec<LispToken>>()
}
