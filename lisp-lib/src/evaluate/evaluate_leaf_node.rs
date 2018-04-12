use evaluate::EvalError;
use parse::LispAstNode;
use lisp_token::LispToken::{
    Variable,
    Value,
};

pub fn evaluate_leaf(node: LispAstNode) -> Result<f64, EvalError> {
    match node.token {
        Variable(_) => unimplemented!(),
        Value(val_str) => {
            let val = val_str.parse::<f64>();
            if val.is_ok() { Ok(val.unwrap())                    }
            else           { Err(EvalError::ValueParseError) }
        },
        _ => Err(EvalError::InvalidLeafNode),
    }
}
