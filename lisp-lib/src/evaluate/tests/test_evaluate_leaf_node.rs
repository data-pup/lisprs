#[cfg(test)]
mod evaluate_leaf_node_tests {
    use evaluate::evaluate_leaf_node::evaluate_leaf;
    use evaluate::EvalError;
    use lisp_token::LispToken;
    use parse::LispAstNode;

    #[test]
    fn leaf_node_that_is_not_var_or_val_causes_error() {
        unimplemented!();
    }

    struct ValueParseTestCase {
        input: &'static str,
        expected: Result<f64, EvalError>,
        desc: &'static str,
    }

    static PARSE_TESTS: &[ValueParseTestCase] = &[
        ValueParseTestCase {
            input: "1", expected: Ok(1_f64), desc: "Parse 1",
        },
        ValueParseTestCase {
            input: "100", expected: Ok(100_f64), desc: "Parse 100",
        },
        ValueParseTestCase {
            input: "1.5", expected: Ok(1.5_f64), desc: "Parse 1.5",
        },
        ValueParseTestCase {
            input: "-1", expected: Ok(-1_f64), desc: "Parse -1",
        },
        ValueParseTestCase {
            input: "-0.005", expected: Ok(-0.005_f64), desc: "Parse -0.005",
        },
    ];

    #[test]
    fn value_node_can_be_parsed_to_f64() {
        for test_case in PARSE_TESTS.iter() {
            let &ValueParseTestCase { input, ref expected, desc } = test_case;
            let node = LispAstNode {
                token: LispToken::Value(String::from(input)),
                children: None,
            };
            let result = evaluate_leaf(node);
            if expected.is_ok() {
                let expected_result = expected.as_ref().unwrap();
                let actual_result = result.unwrap();
                assert_eq!(actual_result, *expected_result,
                    "Test Failed: {}", desc);
            } else {
                let expected_err = expected.as_ref().unwrap_err();
                let actual_err = result.unwrap_err();
                assert_eq!(actual_err, *expected_err, "Test Failed: {}", desc);
            }
        }
    }
}
