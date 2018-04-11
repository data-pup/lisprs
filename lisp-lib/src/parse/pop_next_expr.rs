use lisp_token::LispToken;
use lisp_token::LispToken::{CloseExpression, OpenExpression};

use parse::_LispAstNode;

use parse::_ParseError;
use parse::_ParseError::{
    EmptyExpression,
    UnexpectedOpenParen,
};

impl _LispAstNode {
    /// Pop elements off of the token stack, and return the tokens that form
    /// the next expression in the stack. Returns an error if the stack is
    /// empty, or if there is problem with the parentheses.
    fn get_next_expr(token_stack: &mut Vec<LispToken>)
        -> Result<Vec<LispToken>,_ParseError> {
        let mut expr = vec![];
        while let Some(token) = token_stack.pop() { expr.push(token) }
        Ok(expr)
    }

    fn is_wrapped_expr(token_stack: &Vec<LispToken>)
        -> Result<bool, _ParseError> {
        if let Some(token) = token_stack.last() {
            match token {
                &CloseExpression => Ok(true),
                &OpenExpression  => Err(UnexpectedOpenParen),
                _               => Ok(false),
            }
        } else { Err(EmptyExpression) }
    }
}

#[cfg(test)]
mod get_next_expr_tests {
    use parse::{_LispAstNode, _ParseError};
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

        assert_eq!(_LispAstNode::is_wrapped_expr(&input).unwrap(), false);
        let output = _LispAstNode::get_next_expr(&mut input);

        assert!(input.is_empty());
        assert!(output.is_ok());
        assert_eq!(output.unwrap(), expected);
    }

    #[test]
    fn get_next_expr_handles_wrapped_expr() {
        let mut input = vec![ // (+ 1 2)
            LispToken::OpenExpression,
            LispToken::Operator(LispOperator::Add),
                LispToken::Value(String::from("1")),
                LispToken::Value(String::from("2")),
            LispToken::CloseExpression,
        ];
        let expected = vec![
                LispToken::Value(String::from("2")),
                LispToken::Value(String::from("1")),
            LispToken::Operator(LispOperator::Add),
        ];

        assert_eq!(_LispAstNode::is_wrapped_expr(&input).unwrap(), true);
        let output = _LispAstNode::get_next_expr(&mut input);

        assert!(input.is_empty());
        assert!(output.is_ok());
        assert_eq!(output.unwrap(), expected);
    }
}
