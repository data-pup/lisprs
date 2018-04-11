#[cfg(test)]
mod raw_token_processing_tests {
    use lex::split_raw_token;
    use lisp_operator::LispOperator;
    use lisp_token::LispToken;

    #[test]
    fn check_results() {
        // let result = split_raw_token::process_raw_lisp_token(input.to_string());
        // assert_eq!(result, expected, "Incorrectly split: {}", input);
        for &TestCase { input, ref expected, desc } in get_test_cases().iter() {
            let output = split_raw_token::process_raw_lisp_token(input.to_string());
            assert_eq!(output, *expected, "Failed Test: {}", desc);
        }
    }

    struct TestCase {
        input: &'static str,
        expected: Vec<LispToken>,
        desc: &'static str,
    }

    fn get_test_cases() -> Vec<TestCase> {
        return vec![
            TestCase {
                input:    "(",
                expected: vec![LispToken::OpenExpression],
                desc:     "operator token",
            },
            TestCase {
                input:    "+",
                expected: vec![LispToken::Operator(LispOperator::Add)],
                desc:     "operator token",
            },
            TestCase {
                input:    "10",
                expected: vec![LispToken::Value(String::from("10"))],
                desc:     "value token",
            },
            TestCase {
                input:    "hello",
                expected: vec![LispToken::Variable(String::from("hello"))],
                desc:     "simple example variable name",
            },
            TestCase {
                input:    "World",
                expected: vec![LispToken::Variable(String::from("World"))],
                desc:     "simple example variable name",
            },
            TestCase {
                input: "(+",
                expected: vec![
                    LispToken::OpenExpression,
                    LispToken::Operator(LispOperator::Add),
                ],
                desc: "open parentheses and operator",
            },
            TestCase {
                input: "((",
                expected: vec![
                    LispToken::OpenExpression,
                    LispToken::OpenExpression,
                ],
                desc: "two open parentheses",
            },
            TestCase {
                input: "(1)",
                expected: vec![
                    LispToken::OpenExpression,
                    LispToken::Value(String::from("1")),
                    LispToken::CloseExpression,
                ],
                desc: "enclosed value",
            },
        ];
    }
}
