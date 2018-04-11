#![feature(try_from)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

mod evaluate;
mod lex;
mod lisp_token;
mod lisp_operator;
mod parse;

pub fn get_result(input: &str) -> String {
    let tokens = lex::get_tokens(input);
    let ast = parse::parse(tokens).unwrap();
    evaluate::evaluate(ast)
}
