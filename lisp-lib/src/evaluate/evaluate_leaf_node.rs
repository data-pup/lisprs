use evaluate::EvalError;
use lisp_token::LispToken::{Value, Variable};
use parse::LispAstNode;

pub fn evaluate_leaf(node: LispAstNode) -> Result<f64, EvalError> {
    match node.token {
        Variable(_) => unimplemented!(),
        Value(val_str) => {
            let val = val_str.parse::<f64>();
            if val.is_ok() {
                Ok(val.unwrap())
            } else {
                Err(EvalError::ValueParseError)
            }
        }
        _ => Err(EvalError::InvalidLeafNode),
    }
}
