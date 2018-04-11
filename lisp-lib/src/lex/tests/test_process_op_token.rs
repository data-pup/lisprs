#[cfg(test)]
mod op_processing_tests {
    use lex::process_op_token::process_op_token;
    use lisp_operator::LispOperator;
    use lisp_token::LispToken;

    #[test]
    fn op_processing_works() {
        for test_case in TEST_CASES.iter() { run_test(test_case); }
    }

    fn run_test(test_case: &TestCase) {
        let &TestCase { input, ref expected, desc } = test_case;
        let token = String::from(input);
        let result = process_op_token(&token);
        if expected.is_none() { assert!(result.is_none()) }
        else {
            let actual_token = result.unwrap();
            let expected_token = expected.as_ref().unwrap();
            assert_eq!(actual_token, *expected_token, "Failed Test: {}", desc);
        }
    }

    struct TestCase {
        input:    &'static str,
        expected: Option<LispToken>,
        desc:     &'static str,
    }

    static TEST_CASES: &[TestCase] = &[
        TestCase {
            input:    "+",
            expected: Some(LispToken::Operator(LispOperator::Add)),
            desc:     "Addition",
        },
        TestCase {
            input:    "-",
            expected: Some(LispToken::Operator(LispOperator::Subtract)),
            desc:     "Subtraction",
        },
        TestCase {
            input:    "*",
            expected: Some(LispToken::Operator(LispOperator::Multiply)),
            desc:     "Multiplication",
        },
        TestCase {
            input:    "/",
            expected: Some(LispToken::Operator(LispOperator::Divide)),
            desc:     "Division",
        },
        TestCase { input: "1",     expected: None, desc: "Digit"  },
        TestCase { input: "hello", expected: None, desc: "Symbol" },
        TestCase { input: "!",     expected: None, desc: "Bang"   },
        TestCase { input: ",",     expected: None, desc: "Comma"  },
    ];
}
