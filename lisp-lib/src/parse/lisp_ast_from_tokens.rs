use lisp_operator::LispOperator;
use lisp_token::LispToken;
use parse::_ParseError;
use std::convert::TryFrom;

use parse::LispAstNode;

impl TryFrom<Vec<LispToken>> for LispAstNode {
    type Error = _ParseError;
    fn try_from(tokens: Vec<LispToken>) -> Result<LispAstNode, _ParseError> {
        let mut curr_expr: Vec<LispAstNode> = vec![];
        let mut curr_depth: u8 = 0;

        for curr_token in tokens.iter().rev() {
            match curr_token {
                &LispToken::Operator(op) => {
                    let operands = curr_expr.into_iter().rev().collect::<Vec<LispAstNode>>();
                    let op_node = LispAstNode::create_op_node(op, operands)?;
                    curr_expr = vec![op_node];
                }
                &LispToken::Variable(ref var_token) => {
                    curr_expr.push(LispAstNode::create_var_node(var_token))
                }
                &LispToken::Value(ref val_token) => {
                    curr_expr.push(LispAstNode::create_val_node(val_token))
                }
                &LispToken::OpenExpression if curr_depth == 0 => {
                    return Err(_ParseError::UnexpectedParen)
                }
                &LispToken::OpenExpression if curr_expr.is_empty() => {
                    return Err(_ParseError::EmptyExpression)
                }
                &LispToken::OpenExpression => curr_depth -= 1,
                &LispToken::CloseExpression => curr_depth += 1,
            }
        }

        if curr_depth != 0 {
            Err(_ParseError::UnexpectedParen)
        } else if curr_expr.len() == 0 {
            Err(_ParseError::EmptyExpression)
        } else if curr_expr.len() != 1 {
            Err(_ParseError::InvalidSyntaxTree)
        } else {
            Ok(curr_expr.pop().unwrap())
        }
    }
}

impl LispAstNode {
    // Create an operator node. If args were given, return a parse error.
    fn create_op_node(
        op: LispOperator,
        args: Vec<LispAstNode>,
    ) -> Result<LispAstNode, _ParseError> {
        if args.is_empty() {
            return Err(_ParseError::MissingOperands);
        } else {
            let new_node = LispAstNode {
                token: LispToken::Operator(op),
                children: Some(args),
            };
            Ok(new_node)
        }
    }

    // Process a symbol token, return the new AST node.
    fn create_var_node(var_token: &str) -> LispAstNode {
        LispAstNode {
            token: LispToken::Variable(var_token.to_string()),
            children: None,
        }
    }

    // Create a new value node, return the new AST node.
    fn create_val_node(val_token: &str) -> LispAstNode {
        LispAstNode {
            token: LispToken::Value(val_token.to_string()),
            children: None,
        }
    }
}
