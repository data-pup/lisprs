use lisp_token::LispToken;

use lex::process_op_token::process_op_token;
use lex::split_parens::split_paren_token;
use lex::token_regex::{is_op_token, is_value_token, is_variable_token};

/// Process a raw Lisp token, return a collection of the resulting LispToken
/// objects. Note that this is a vector, because a raw token such as '(('
/// represents a sequence of syntactically meaningful tokens.
pub fn process_raw_lisp_token(token: String) -> Vec<LispToken> {
    match &token {
        raw_var if is_variable_token(&token) => {
            let var = LispToken::Variable(raw_var.clone());
            vec![var]
        }
        raw_val if is_value_token(&token) => {
            let val = LispToken::Value(raw_val.clone());
            vec![val]
        }
        raw_op if is_op_token(&token) => {
            let op = process_op_token(&raw_op).unwrap();
            vec![op]
        }
        compl_token if token.len() > 1 => split_paren_token(compl_token)
            .into_iter()
            .flat_map(process_raw_lisp_token)
            .collect(),
        paren_token if token.len() == 1 => {
            let c: char = paren_token.chars().next().unwrap();
            match c {
                '(' => vec![LispToken::OpenExpression],
                ')' => vec![LispToken::CloseExpression],
                _ => panic!("Unexpected token!"),
            }
        }
        _ => panic!("Unexpected empty raw token!"),
    }
}
