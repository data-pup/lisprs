mod eval_error;
mod evaluate_leaf_node;
mod tests;

pub use self::eval_error::EvalError;
use self::evaluate_leaf_node::evaluate_leaf;

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

fn evaluate_expr(_node: LispAstNode) -> Result<f64, EvalError> {
    unimplemented!();
}
