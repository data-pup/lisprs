use std::iter::FromIterator;

use lisp_token::{
    LispToken,
    OperatorToken,
};

use lex::token_regex::{
    is_syntax_char,
    is_value_token,
    is_variable_token,
};

/// Split a raw token into its syntactically meaningful components.
pub fn process_raw_token(token: String) -> Vec<String> {
    match &token {
        t if is_variable_token(&t) || is_value_token(&t) => vec![token.clone()],
        t => { // Otherwise, process the components of the token.
            let mut tokens: Vec<String> = vec![];
            let mut curr_token: Vec<char> = vec![];
            for curr_c in t.chars() {
                match curr_c {
                    c if is_syntax_char(c) => {
                        if !&curr_token.is_empty() {
                            tokens.push(String::from_iter(curr_token));
                            curr_token = vec![];
                        }
                        tokens.push(curr_c.to_string())
                    },
                    _ => curr_token.push(curr_c),
                }
            }
            if !&curr_token.is_empty() { tokens.push(String::from_iter(curr_token)); }
            return tokens;
        }
    }
}

pub fn process_raw_token_new_impl(token: String) -> Vec<LispToken> {
    match &token {
        raw_var if is_variable_token(&token) => {
            let var = LispToken::Variable(raw_var.clone());
            vec![var]
        },
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod raw_token_processing_tests {
    use lex::split_raw_token;

    /// Helper function that, given an input string and a vector containing the
    /// expected results of the process_raw_token function, checks output.
    fn check_results(input: &'static str, expected: Vec<String>) {
        let result = split_raw_token::process_raw_token(input.to_string());
        assert_eq!(result, expected, "Incorrectly split: {}", input);
    }

    #[test]
    fn basic_tokens_should_be_returned_identically() {
        let test_cases = &["hello", "+", "(", "0", "10"].iter()
            .map(|s| (*s, vec![s.to_string()]))
            .collect::<Vec<(&'static str, Vec<String>)>>();
        for &(input, ref expected) in test_cases.into_iter() {
            check_results(input, expected.clone());
        }
    }

    #[test]
    fn raw_tokens_containing_parens_should_be_split() {
        let test_cases = &[
            ("(+", vec![String::from("("), String::from("+")]),
            ("((", vec![String::from("("), String::from("(")]),
            ("(((", vec![
                        String::from("("),
                        String::from("("),
                        String::from("(")
                    ]
            ),
            ("(1)", vec![
                        String::from("("),
                            String::from("1"),
                        String::from(")")
                    ]
            ),
        ];
        for &(input, ref expected) in test_cases.into_iter() {
            check_results(input, expected.clone());
        }
    }
}

#[cfg(test)]
mod new_impl_tests {
    use lex::split_raw_token;
    use lisp_token::{LispToken, OperatorToken};

    #[test]
    fn simple_tokens_should_be_identified_properly() {
        let test_cases = &[
            ("+", vec![LispToken::Operator(OperatorToken::Add)]),
            ("(", vec![LispToken::OpenExpression]),
            (")", vec![LispToken::CloseExpression]),
            ("10", vec![LispToken::Value(String::from("10"))]),
            ("hello", vec![LispToken::Variable(String::from("hello"))]),
        ];
        for &(input_str, ref expected) in test_cases.into_iter() {
            let output = split_raw_token::process_raw_token_new_impl(
                input_str.to_string());
            assert_eq!(output, *expected,
                "Lisp Token Could Not Be Identified: {}", input_str);
        }
    }
}
