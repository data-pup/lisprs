#[macro_use]
extern crate lazy_static;
extern crate regex;

mod lex;
mod lisp_token;
mod parse;

pub fn get_result(input: &str) -> String {
    let tokens = lex::get_tokens(input);
    unimplemented!();
    // parse::parse(tokens)
}
