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
    use lisp_operator::LispOperator;
    use lisp_token::LispToken;

    #[test]
    fn get_tokens_should_split_simple_expr_by_whitespace() {
        let input = "( + 1 1 )";
        let result = lex::get_tokens(input);
        let expected = vec![
            LispToken::OpenExpression,
                LispToken::Operator(LispOperator::Add),
                    LispToken::Value(String::from("1")),
                    LispToken::Value(String::from("1")),
            LispToken::CloseExpression,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tokens_should_split_basic_expr_by_whitespace() {
        let input = "(+ 1 1)";
        let result = lex::get_tokens(input);
        let expected = vec![
            LispToken::OpenExpression,
                LispToken::Operator(LispOperator::Add),
                    LispToken::Value(String::from("1")),
                    LispToken::Value(String::from("1")),
            LispToken::CloseExpression,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tokens_should_handle_a_simple_nested_expression() {
        let input = "(+ (* 1 2) 3 (4))";
        let result = lex::get_tokens(input);
        let expected: Vec<LispToken> = vec![
            LispToken::OpenExpression,
                LispToken::Operator(LispOperator::Add),
                    LispToken::OpenExpression,
                        LispToken::Operator(LispOperator::Multiply),
                        LispToken::Value(String::from("1")),
                        LispToken::Value(String::from("2")),
                    LispToken::CloseExpression,
                    LispToken::Value(String::from("3")),
                    LispToken::OpenExpression,
                    LispToken::Value(String::from("4")),
                    LispToken::CloseExpression,
            LispToken::CloseExpression,
        ];
        assert_eq!(result, expected);
    }
}
