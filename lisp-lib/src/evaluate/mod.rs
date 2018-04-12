mod eval_error;
mod evaluate_leaf_node;
mod tests;

pub use self::eval_error::EvalError;
use self::evaluate_leaf_node::evaluate_leaf;

use lisp_operator::LispOperator::{Add, Subtract, Multiply, Divide};
use lisp_token::LispToken::Operator;
use parse::LispAstNode;

pub fn evaluate(ast: LispAstNode) -> String {
    match evaluate_helper(ast) {
        Ok(_) => unimplemented!(),
        Err(_) => String::from("Error!"),
    }
}

fn evaluate_helper(ast: LispAstNode) -> Result<f64, EvalError> {
    match ast.has_children() {
        true => evaluate_expr(ast),
        false => evaluate_leaf(ast),
    }
}

fn evaluate_expr(node: LispAstNode) -> Result<f64, EvalError> {
    if let Some(operands) = node.children {
        let operand_values: Vec<f64> =
            operands
            .into_iter()
            .map(evaluate_helper)
            .collect::<Result<Vec<f64>, EvalError>>()?;
        match node.token {
            Operator(op) => {
                match op {
                    Add => Ok(operand_values.into_iter().fold(0_f64, |res, i| res + i)),
                    Subtract => unimplemented!(),
                    Multiply => unimplemented!(),
                    Divide => unimplemented!(),
                }
            }
            _ => Err(EvalError::InvalidOp),
        }
    } else {
        Err(EvalError::MissingOperands)
    }
}
