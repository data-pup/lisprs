mod split_raw_token;
mod token_regex;

use lisp_token::LispToken;

pub fn get_tokens(input: &str) -> Vec<LispToken> {
    let raw_input = input.to_string();
    raw_input.split_whitespace()
        .map(str::to_string)
        .flat_map(split_raw_token::process_raw_lisp_token)
        .collect::<Vec<LispToken>>()
}

#[cfg(test)]
mod lex_tests {
    use lex;
    use lisp_token::{LispToken, OperatorToken};

    // FIXUP: Enable tests once refactoring is complete.

    // #[test]
    // fn get_tokens_should_split_simple_expr_by_whitespace() {
    //     let input = "( + 1 1 )";
    //     let result = lex::get_tokens(input);
    //     let expected = vec![
    //         LispToken::OpenExpression,
    //             LispToken::Operator(OperatorToken::Add),
    //                 LispToken::Value(String::from("1")),
    //                 LispToken::Value(String::from("1")),
    //         LispToken::CloseExpression,
    //     ];
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn get_tokens_should_split_basic_expr_by_whitespace() {
    //     let input = "(+ 1 1)";
    //     let result = lex::get_tokens(input);
    //     let expected = vec![
    //         String::from("("),
    //             String::from("+"),
    //                 String::from("1"),
    //                 String::from("1"),
    //         String::from(")"),
    //     ];
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn get_tokens_should_handle_a_simple_nested_expression() {
    //     let input = "(+ (* 1 2) 3 (4))";
    //     let result = lex::get_tokens(input);
    //     let expected: Vec<String> = vec![
    //         "(",
    //             "+",
    //                 "(", "*", "1", "2", ")",
    //                 "3",
    //                 "(", "4", ")",
    //         ")",
    //     ].into_iter().map(str::to_string).collect();
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn get_tokens_should_handle_simple_nested_values() {
    //     let input = "(+ (1))";
    //     let result = lex::get_tokens(input);
    //     let expected: Vec<String> = vec![
    //         "(",
    //             "+",
    //                 "(", "1", ")",
    //         ")",
    //     ].into_iter().map(str::to_string).collect();
    //     assert_eq!(result, expected);
    // }

}
