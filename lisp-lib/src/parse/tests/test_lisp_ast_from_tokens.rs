#[cfg(test)]
mod lisp_ast_from_tokens_tests {
    use std::convert::TryFrom;

    use lisp_operator::LispOperator;
    use lisp_token::LispToken;
    use parse::_ParseError;
    use parse::lisp_ast::LispAstNode;

    #[test]
    fn op_with_no_children_returns_error() {
        let invalid_exprs = vec![
            vec![
                // ( + )
                LispToken::OpenExpression,
                LispToken::Operator(LispOperator::Add),
                LispToken::CloseExpression,
            ],
            vec![
                // +
                LispToken::Operator(LispOperator::Add),
            ],
        ];
        for curr_expr in invalid_exprs.into_iter() {
            let result = LispAstNode::try_from(curr_expr);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), _ParseError::MissingOperands);
        }
    }

    #[test]
    fn invalid_parens_return_error() {
        let invalid_exprs = vec![
            vec![LispToken::OpenExpression],  // (
            vec![LispToken::CloseExpression], // )
            vec![
                // ) 1 (
                LispToken::CloseExpression,
                LispToken::Value(String::from("1")),
                LispToken::OpenExpression,
            ],
            vec![
                // (( 1)
                LispToken::OpenExpression,
                LispToken::OpenExpression,
                LispToken::Value(String::from("1")),
                LispToken::CloseExpression,
            ],
            vec![
                // ( 1 ))
                LispToken::OpenExpression,
                LispToken::Value(String::from("1")),
                LispToken::CloseExpression,
                LispToken::CloseExpression,
            ],
        ];
        for curr_expr in invalid_exprs.into_iter() {
            let result = LispAstNode::try_from(curr_expr);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), _ParseError::UnexpectedParen);
        }
    }

    #[test]
    fn empty_expressions_cause_error() {
        let invalid_exprs = vec![
            vec![],
            vec![LispToken::OpenExpression, LispToken::CloseExpression],
            vec![
                // (+ 1 ())
                LispToken::OpenExpression,
                LispToken::Operator(LispOperator::Add),
                LispToken::Value(String::from("1")),
                LispToken::OpenExpression,
                LispToken::CloseExpression,
                LispToken::CloseExpression,
            ],
        ];
        for curr_expr in invalid_exprs.into_iter() {
            let result = LispAstNode::try_from(curr_expr);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), _ParseError::EmptyExpression);
        }
    }

    #[test]
    fn simple_expression_parses_correctly() {
        let input = vec![
            LispToken::OpenExpression,
            LispToken::Operator(LispOperator::Add),
            LispToken::Value(String::from("1")),
            LispToken::Value(String::from("2")),
            LispToken::CloseExpression,
        ];
        let expected = LispAstNode {
            token: LispToken::Operator(LispOperator::Add),
            children: Some(vec![
                LispAstNode {
                    token: LispToken::Value(String::from("1")),
                    children: None,
                },
                LispAstNode {
                    token: LispToken::Value(String::from("2")),
                    children: None,
                },
            ]),
        };
        let result = LispAstNode::try_from(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn nested_expression_parses_correctly() {
        let input = vec![
            LispToken::OpenExpression,
            LispToken::Operator(LispOperator::Add),
            LispToken::Value(String::from("1")),
            LispToken::OpenExpression,
            LispToken::Operator(LispOperator::Add),
            LispToken::Value(String::from("2")),
            LispToken::Value(String::from("3")),
            LispToken::CloseExpression,
            LispToken::CloseExpression,
        ];
        let expected = LispAstNode {
            token: LispToken::Operator(LispOperator::Add),
            children: Some(vec![
                LispAstNode {
                    token: LispToken::Value(String::from("1")),
                    children: None,
                },
                LispAstNode {
                    token: LispToken::Operator(LispOperator::Add),
                    children: Some(vec![
                        LispAstNode {
                            token: LispToken::Value(String::from("2")),
                            children: None,
                        },
                        LispAstNode {
                            token: LispToken::Value(String::from("3")),
                            children: None,
                        },
                    ]),
                },
            ]),
        };
        let result = LispAstNode::try_from(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
