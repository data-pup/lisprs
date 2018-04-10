use std::convert::TryFrom;

use lisp_token::LispToken;
use parse::_ParseError;

type ChildList = Option<Box<Vec<_LispAstNode>>>;

#[derive(Debug)]
struct _LispAstNode {
    token: LispToken,
    children: Option<Vec<_LispAstNode>>,
}

impl TryFrom<Vec<LispToken>> for _LispAstNode {
    type Error = _ParseError;
    fn try_from(tokens: Vec<LispToken>) -> Result<_LispAstNode, _ParseError> {
        let mut child_nodes: Vec<LispToken> = vec![];
        let mut curr_depth: u8 = 0;
        for curr_token in tokens.iter().rev() {
            match curr_token {
                // Operator(LispOperator) => _li,
                // Variable(String),
                // Value(String),
                &LispToken::OpenExpression if curr_depth == 0 => {
                    return Err(_ParseError::UnexpectedParen);
                }
                &LispToken::OpenExpression => {
                    // let new_node = _LispAstNode{ token: curr_token, children: child_nodes };
                    // child_nodes = vec![new_node];
                    curr_depth -= 1;
                }
                &LispToken::CloseExpression => curr_depth += 1,
                _ => unimplemented!(),
            }
        }

        if curr_depth != 1 { return Err(_ParseError::UnexpectedParen); }

        unimplemented!();
    }
}

#[cfg(test)]
mod node_tests {
    use std::convert::TryFrom;
    use parse::lisp_ast;
    use parse::_ParseError;
    use lisp_token::LispToken;

    #[test]
    fn invalid_parens_return_error() {
        let input = vec![
            LispToken::OpenExpression,
            LispToken::OpenExpression,
            LispToken::OpenExpression,
            LispToken::CloseExpression,
        ];
        let result = lisp_ast::_LispAstNode::try_from(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), _ParseError::UnexpectedParen);
    }
}
