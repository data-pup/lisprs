use parse::LispAstNode;

enum EvaluateError {
    TempErrorName,
}

pub fn evaluate(ast: LispAstNode) -> String {
    match evaluate_helper(ast) {
        Ok(res) => unimplemented!(),
        Err(_) => String::from("Error!"),
    }
}

fn evaluate_helper(ast: LispAstNode) -> Result<f64, EvaluateError> {
    match ast.has_children() {
        true => evaluate_expr(ast),
        false => evaluate_leaf_node(ast),
    }
}

fn evaluate_expr(_node: LispAstNode) -> Result<f64, EvaluateError> {
    unimplemented!();
}

fn evaluate_leaf_node(_node: LispAstNode) -> Result<f64, EvaluateError> {
    unimplemented!();
}
