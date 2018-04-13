#[cfg(test)]
mod valid_leaf_node_tests {
    use evaluate::evaluate_leaf_node::evaluate_leaf;
    use evaluate::EvalError;
    use lisp_token::LispToken;
    use parse::LispAstNode;

    #[test]
    fn value_node_can_be_parsed_to_f64() {
        for test_case in VALUE_PARSE_TESTS.iter() {
            let &ValueParseTestCase {
                input,
                ref expected,
                desc,
            } = test_case;
            let node = LispAstNode {
                token: LispToken::Value(String::from(input)),
                children: None,
            };
            let result = evaluate_leaf(node);
            if expected.is_ok() {
                let expected_result = expected.as_ref().unwrap();
                let actual_result = result.unwrap();
                assert_eq!(actual_result, *expected_result, "Test Failed: {}", desc);
            } else {
                let expected_err = expected.as_ref().unwrap_err();
                let actual_err = result.unwrap_err();
                assert_eq!(actual_err, *expected_err, "Test Failed: {}", desc);
            }
        }
    }

    struct ValueParseTestCase {
        input: &'static str,
        expected: Result<f64, EvalError>,
        desc: &'static str,
    }

    static VALUE_PARSE_TESTS: &[ValueParseTestCase] = &[
        ValueParseTestCase {
            input: "1",
            expected: Ok(1_f64),
            desc: "Parse 1",
        },
        ValueParseTestCase {
            input: "100",
            expected: Ok(100_f64),
            desc: "Parse 100",
        },
        ValueParseTestCase {
            input: "1.5",
            expected: Ok(1.5_f64),
            desc: "Parse 1.5",
        },
        ValueParseTestCase {
            input: "-1",
            expected: Ok(-1_f64),
            desc: "Parse -1",
        },
        ValueParseTestCase {
            input: "-0.005",
            expected: Ok(-0.005_f64),
            desc: "Parse -0.005",
        },
    ];
}

#[cfg(test)]
mod invalid_leaf_node_tests {
    use evaluate::evaluate_leaf_node::evaluate_leaf;
    use evaluate::EvalError;
    use lisp_operator::LispOperator;
    use lisp_token::LispToken;
    use parse::LispAstNode;

    #[test]
    fn invalid_leaf_nodes_cause_error() {
        for test_case in init_test_cases().into_iter() {
            let InvalidLeafTestCase {
                input,
                expected,
                desc,
            } = test_case;
            let result = evaluate_leaf(input);
            assert!(result.is_err());
            let actual_err = result.unwrap_err();
            assert_eq!(actual_err, expected, "Test Failed: {}", desc);
        }
    }

    fn init_test_cases() -> Vec<InvalidLeafTestCase> {
        vec![
            InvalidLeafTestCase {
                input: LispAstNode {
                    token: LispToken::CloseExpression,
                    children: None,
                },
                expected: EvalError::InvalidLeafNode,
                desc: "Closing parentheses cannot be leaf node.",
            },
            InvalidLeafTestCase {
                input: LispAstNode {
                    token: LispToken::OpenExpression,
                    children: None,
                },
                expected: EvalError::InvalidLeafNode,
                desc: "Opening parentheses cannot be leaf node.",
            },
            InvalidLeafTestCase {
                input: LispAstNode {
                    token: LispToken::Operator(LispOperator::Add),
                    children: None,
                },
                expected: EvalError::InvalidLeafNode,
                desc: "Operator cannot be leaf node.",
            },
            InvalidLeafTestCase {
                input: LispAstNode {
                    token: LispToken::Value(String::from("Not a value!")),
                    children: None,
                },
                expected: EvalError::ValueParseError,
                desc: "Operator cannot be leaf node.",
            },
        ]
    }

    struct InvalidLeafTestCase {
        input: LispAstNode,
        expected: EvalError,
        desc: &'static str,
    }
}
