mod lex;
mod parse;

use lex::get_tokens;
use parse::parse;

pub fn get_result(input: &str) -> &str {
    let tokens = get_tokens(input);
    parse::parse(tokens)
}
