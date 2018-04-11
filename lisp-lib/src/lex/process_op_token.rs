use lisp_operator::LispOperator;
use lisp_token::LispToken;

pub fn process_op_token(token: &str) -> Option<LispToken> {
    match token {
        "+" => Some(LispToken::Operator(LispOperator::Add)),
        "-" => Some(LispToken::Operator(LispOperator::Subtract)),
        "*" => Some(LispToken::Operator(LispOperator::Multiply)),
        "/" => Some(LispToken::Operator(LispOperator::Divide)),
        _ => None,
    }
}
