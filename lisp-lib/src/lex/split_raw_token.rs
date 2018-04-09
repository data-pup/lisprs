use std::iter::FromIterator;

use lisp_token::{
    LispToken,
    // OperatorToken,
};

use lex::token_regex::{
    is_op_token,
    is_paren,
    is_syntax_char,
    is_value_token,
    is_variable_token,
};

/// Split a raw token into its syntactically meaningful components.
// pub fn process_raw_token(token: String) -> Vec<String> {
//     match &token {
//         t if is_variable_token(&t) || is_value_token(&t) => vec![token.clone()],
//         t => { // Otherwise, process the components of the token.
//             let mut tokens: Vec<String> = vec![];
//             let mut curr_token: Vec<char> = vec![];
//             for curr_c in t.chars() {
//                 match curr_c {
//                     c if is_syntax_char(c) => {
//                         if !&curr_token.is_empty() {
//                             tokens.push(String::from_iter(curr_token));
//                             curr_token = vec![];
//                         }
//                         tokens.push(curr_c.to_string())
//                     },
//                     _ => curr_token.push(curr_c),
//                 }
//             }
//             if !&curr_token.is_empty() { tokens.push(String::from_iter(curr_token)); }
//             return tokens;
//         }
//     }
// }

/// TEMP: This is a new implementation of the raw token processing that will
/// return elements in the form of a LispToken, rather than a String.
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
            unimplemented!("Operators not implemented yet!");
        },
        paren_token => {
            split_paren_token(paren_token)
                .into_iter()
                .flat_map(process_raw_token_new_impl)
                .collect()
        }
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
