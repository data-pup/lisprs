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
                &LispToken::OpenExpression if curr_expr.is_empty() =>
                    return Err(_ParseError::EmptyExpression),
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
