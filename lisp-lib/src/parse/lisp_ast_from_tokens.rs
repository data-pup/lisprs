use std::convert::TryFrom;
use lisp_operator::LispOperator;
use lisp_token::LispToken;
use parse::_ParseError;

use parse::_LispAstNode;

impl TryFrom<Vec<LispToken>> for _LispAstNode {
    type Error = _ParseError;
    fn try_from(tokens: Vec<LispToken>) -> Result<_LispAstNode, _ParseError> {
        let mut curr_expr: Vec<_LispAstNode> = vec![];
        let mut curr_depth: u8 = 0;
        for curr_token in tokens.iter().rev() {
            match curr_token {
                &LispToken::Operator(op) =>
                    curr_expr = vec![_LispAstNode::create_op_node(op, curr_expr)?],
                &LispToken::Variable(ref var_token) =>
                    curr_expr.push(_LispAstNode::create_var_node(var_token)),
                &LispToken::Value(ref val_token) =>
                    curr_expr.push(_LispAstNode::create_val_node(val_token)),
                &LispToken::OpenExpression if curr_depth == 0 =>
                    return Err(_ParseError::UnexpectedParen),
                &LispToken::OpenExpression => curr_depth -= 1,
                &LispToken::CloseExpression => curr_depth += 1,
            }
        }

        if curr_depth != 0 { return Err(_ParseError::UnexpectedParen); }
        else if curr_expr.len() == 0 { return Err(_ParseError::EmptyExpression); }
        else if curr_expr.len() != 1 { return Err(_ParseError::InvalidSyntaxTree); }
        else { unimplemented!(); }
    }
}

impl _LispAstNode {
    /// Convert a collection of lisp tokens into a collection of child nodes.
    fn parse_operand_tokens(operands: &[LispToken])
        -> Result<Vec<_LispAstNode>, _ParseError> {
        operands.iter()
        .map(|node| {
            match node {
                &LispToken::Variable(ref var_token) =>
                    Ok(_LispAstNode::create_var_node(var_token)),
                &LispToken::Value(ref val_token) =>
                    Ok(_LispAstNode::create_val_node(val_token)),
                _ => return Err(_ParseError::UnexpectedToken),
            }
        })
        .collect()
    }

    // Create an operator node. If args were given, return a parse error.
    fn create_op_node(op: LispOperator, args: Vec<_LispAstNode>)
        -> Result<_LispAstNode, _ParseError> {
        if args.is_empty() { return Err(_ParseError::MissingOperands); }
        else {
            let new_node = _LispAstNode {
                token: LispToken::Operator(op),
                children: Some(args),
            };
            Ok(new_node)
        }
    }

    // Process a symbol token, return the new AST node.
    fn create_var_node(var_token: &str) -> _LispAstNode {
        _LispAstNode {
            token: LispToken::Variable(var_token.to_string()),
            children: None,
        }
    }

    // Create a new value node, return the new AST node.
    fn create_val_node(val_token: &str) -> _LispAstNode {
        _LispAstNode {
            token: LispToken::Value(val_token.to_string()),
            children: None,
        }
    }
}

#[cfg(test)]
mod parse_tests {
    use std::convert::TryFrom;
    use parse::lisp_ast;
    use parse::_ParseError;
    use lisp_token::LispToken;
    use lisp_operator::LispOperator;

    #[test]
    fn op_with_no_children_returns_error() {
        let invalid_exprs = vec![
            vec![
                LispToken::OpenExpression,
                LispToken::Operator(LispOperator::Add),
                LispToken::CloseExpression,
            ],
            vec![
                LispToken::Operator(LispOperator::Add),
            ],
        ];
        for curr_expr in invalid_exprs.into_iter() {
            let result = lisp_ast::_LispAstNode::try_from(curr_expr);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), _ParseError::MissingOperands);
        }
    }

    #[test]
    fn invalid_parens_return_error() {
        let invalid_exprs = vec![
            vec![LispToken::OpenExpression],
            vec![LispToken::CloseExpression],
            vec![
                LispToken::CloseExpression,
                LispToken::Value(String::from("1")),
                LispToken::OpenExpression
            ],
            vec![
                LispToken::OpenExpression,
                LispToken::OpenExpression,
                LispToken::Value(String::from("1")),
                LispToken::CloseExpression,
            ],
            vec![
                LispToken::OpenExpression,
                LispToken::Value(String::from("1")),
                LispToken::CloseExpression,
                LispToken::CloseExpression,
            ],
        ];
        for curr_expr in invalid_exprs.into_iter() {
            let result = lisp_ast::_LispAstNode::try_from(curr_expr);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), _ParseError::UnexpectedParen);
        }
    }

    #[test]
    fn empty_expressions_cause_error() {
        let invalid_exprs = vec![
            vec![],
            vec![LispToken::OpenExpression, LispToken::CloseExpression],
            vec![ // (+ 1 ())
                LispToken::OpenExpression,
                LispToken::Operator(LispOperator::Add),
                LispToken::Value(String::from("1")),
                LispToken::OpenExpression,
                LispToken::CloseExpression,
                LispToken::CloseExpression,
            ],
        ];
        for curr_expr in invalid_exprs.into_iter() {
            let result = lisp_ast::_LispAstNode::try_from(curr_expr);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), _ParseError::EmptyExpression);
        }
    }

    #[test]
    fn operand_tokens_are_parsed_correctly() {
        let input = vec![
            LispToken::Value(String::from("2")),
            LispToken::Variable(String::from("variable")),
        ];
        let expected = vec![
            lisp_ast::_LispAstNode {
                token: LispToken::Value(String::from("2")),
                children: None,
            },
            lisp_ast::_LispAstNode {
                token: LispToken::Variable(String::from("variable")),
                children: None,
            },
        ];
        let output = lisp_ast::_LispAstNode::parse_operand_tokens(&input);
        assert_eq!(output.unwrap(), expected);
    }
}
