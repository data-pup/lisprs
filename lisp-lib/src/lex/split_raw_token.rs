use std::iter::FromIterator;

use lisp_token::{
    LispToken,
    OperatorToken,
};

use lex::token_regex::{
    is_op_token,
    is_paren,
    is_value_token,
    is_variable_token,
};

/// TEMP: This is a new implementation of the raw token processing that will
/// return elements in the form of a LispToken, rather than a String.
/// RENAME ONCE COMPLETE.
pub fn process_raw_token_new_impl(token: String) -> Vec<LispToken> {
    match &token {
        raw_var if is_variable_token(&token) => {
            let var = LispToken::Variable(raw_var.clone());
            vec![var]
        },
        raw_val if is_value_token(&token) => {
            let val = LispToken::Value(raw_val.clone());
            vec![val]
        },
        raw_op if is_op_token(&token) => {
            let op = process_op_token(&token).unwrap();
            vec![op]
        },
        paren_token => {
            split_paren_token(paren_token)
                .into_iter()
                .flat_map(process_raw_token_new_impl)
                .collect()
        }
    }
}

fn process_op_token(token: &str) -> Option<LispToken> {
    match token {
        "+" => Some(LispToken::Operator(OperatorToken::Add)),
        "-" => Some(LispToken::Operator(OperatorToken::Subtract)),
        "*" => Some(LispToken::Operator(OperatorToken::Multiply)),
        "/" => Some(LispToken::Operator(OperatorToken::Divide)),
        _ => None,
    }
}

pub fn split_paren_token(token: &str) -> Vec<String> {
    let add_curr_if_exists =
        |curr: &mut Vec<char>, res: &mut Vec<String>| {
            if !curr.is_empty() {
                let new_token = String::from_iter(curr.clone());
                res.push(new_token);
                curr.clear();
            }
    };

    let (mut curr, mut results) = (vec![], vec![]);
    for c in token.chars() {
        if is_paren(c) {
            add_curr_if_exists(&mut curr, &mut results);
            results.push(c.to_string());
        } else {
            curr.push(c);
        }
    }
    add_curr_if_exists(&mut curr, &mut results);

    return results;
}

#[cfg(test)]
mod raw_token_processing_tests {
    use lex::split_raw_token;
    use lisp_token::{LispToken, OperatorToken};

    #[test]
    fn check_results() {
        // let result = split_raw_token::process_raw_token_new_impl(input.to_string());
        // assert_eq!(result, expected, "Incorrectly split: {}", input);
        for &TestCase { input, ref expected, desc } in get_test_cases().iter() {
            let output = split_raw_token::process_raw_token_new_impl(input.to_string());
            assert_eq!(output, *expected);
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
            // TestCase {
            //     input:    "+",
            //     expected: vec![LispToken::Operator(OperatorToken::Add)],
            //     desc:     "operator token",
            // },
            TestCase {
                input:    "10",
                expected: vec![LispToken::Value(String::from("10"))],
                desc:     "value token",
            },
            // TestCase {
            //     input:    "hello",
            //     expected: &[LispToken::Variable(String::from("hello"))],
            //     desc:     "simple example variable name",
            // },
            // TestCase {
            //     input:    "World",
            //     expected: &[LispToken::Variable(String::from("hello"))],
            //     desc:     "simple example variable name",
            // },
            // TestCase {
            //     input: "(+",
            //     expected: &[
            //         LispToken::OpenExpression,
            //         LispToken::Operator(OperatorToken::Add),
            //     ],
            //     desc: "open parentheses and operator",
            // },
            // TestCase {
            //     input: "((",
            //     expected: &[
            //         LispToken::OpenExpression,
            //         LispToken::OpenExpression,
            //     ],
            //     desc: "two open parentheses",
            // },
            // TestCase {
            //     input: "(1)",
            //     expected: &[
            //         LispToken::OpenExpression,
            //         LispToken::Value(String::from("1")),
            //         LispToken::CloseExpression,
            //     ],
            //     desc: "enclosed value",
            // },
        ];
    }
}

#[cfg(test)]
mod split_parens_tests {
    use lex::split_raw_token;
    #[test]
    fn split_parens_works() {
        let test_cases = vec![
            ("(", vec![String::from("(")]),
            ("(+", vec![String::from("("), String::from("+")]),
            ("+)", vec![String::from("+"), String::from(")")]),
            ("hello)", vec![String::from("hello"), String::from(")")]),
            ("(hello)", vec![String::from("("), String::from("hello"), String::from(")")]),
        ];
        for (input, expected) in test_cases.into_iter() {
            let output = split_raw_token::split_paren_token(input);
            assert_eq!(output, expected, "Test Failed: {}", input);
        }
    }
}

#[cfg(test)]
mod op_processing_tests {
    use lex::split_raw_token;
    use lisp_token::{LispToken, OperatorToken};
    #[test]
    fn op_processing_works() {
        let add_token = String::from("+"); // Check that "+" works.
        let add_op = split_raw_token::process_op_token(&add_token).unwrap();
        assert_eq!(add_op, LispToken::Operator(OperatorToken::Add), "Failed + test.");

        let sub_token = String::from("-"); // Check that "-" works.
        let sub_op = split_raw_token::process_op_token(&sub_token).unwrap();
        assert_eq!(sub_op, LispToken::Operator(OperatorToken::Subtract), "Failed - test.");

        let mult_token = String::from("*"); // Check that "*" works.
        let mult_op = split_raw_token::process_op_token(&mult_token).unwrap();
        assert_eq!(mult_op, LispToken::Operator(OperatorToken::Multiply), "Failed * test.");

        let div_token = String::from("/"); // Check that "/" works.
        let div_op = split_raw_token::process_op_token(&div_token).unwrap();
        assert_eq!(div_op, LispToken::Operator(OperatorToken::Divide), "Failed / test.");

        let invalid_op_token = String::from("$");
        let div_op = split_raw_token::process_op_token(&invalid_op_token);
        assert!(div_op.is_none());
    }
}