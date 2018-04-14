#![feature(try_from)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

mod evaluate;
mod lex;
mod lisp_operator;
mod lisp_token;
mod lisp_value;
mod lisp_variable;
mod parse;

pub fn get_result(input: &str) -> String {
    let tokens = lex::get_tokens(input);
    match parse::parse(tokens) {
        Ok(ast) => evaluate::evaluate(ast),
        Err(e) => return format!("Error while parsing: {:?}", e),
    }
}
