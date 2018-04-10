use lisp_token::LispToken;
use parse::{
    _LispAstNode,
    _ParseError,
};

impl _LispAstNode {
    /// Pop tokens off of the stack, and return the tokens that form the next
    /// expression in the stack.
    fn get_next_expr(token_stack: &mut Vec<LispToken>)
        -> Result<Vec<LispToken>,_ParseError> {
        let mut expr = vec![];
        while let Some(token) = token_stack.pop() { expr.push(token) }
        Ok(expr)
    }
}

#[cfg(test)]
mod get_next_expr_tests {
    use parse::lisp_ast;
    use parse::_ParseError;
    use lisp_token::LispToken;
    use lisp_operator::LispOperator;

    #[test]
    fn get_next_expr_handles_unwrapped_expr() {
        let mut input = vec![ // + 1 2
            LispToken::Operator(LispOperator::Add),
                LispToken::Value(String::from("1")),
                LispToken::Value(String::from("2")),
        ];
        let expected = vec![
                LispToken::Value(String::from("2")),
                LispToken::Value(String::from("1")),
            LispToken::Operator(LispOperator::Add),
        ];
        let output = lisp_ast::_LispAstNode::get_next_expr(&mut input);
        assert!(output.is_ok());
        assert_eq!(output.unwrap(), expected);
    }
}
