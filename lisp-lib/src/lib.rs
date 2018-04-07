extern crate regex;

mod lex;
mod parse;

pub fn get_result(input: &str) -> String {
    let tokens = lex::get_tokens(input);
    parse::parse(tokens)
}
