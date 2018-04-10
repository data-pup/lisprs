mod lisp_ast;

use lisp_token::LispToken;

#[derive(Debug, PartialEq)]
enum _ParseError {
    EmptyExpression,
    MissingOpenParen,
    UnexpectedParen,
    MissingOperands,
}

// Parse a vector of tokens, evaluate the expression, and return the result.
pub fn parse(_tokens: Vec<LispToken>) -> String {
    unimplemented!();
}

/// Evaluate an expression given as a vector of strings.
fn _evaluate(_args: Vec<String>) -> String {
    unimplemented!();
}

#[cfg(test)]
mod parse_tests {
    // use parse;

    // #[test]
    // fn parse_handles_basic_expression() {
    //     let input = vec![
    //         String::from("("),
    //             String::from("+"), String::from("1"), String::from("1"),
    //         String::from(")")
    //     ];
    //     let result: String = parse::parse(input);
    //     let expected = String::from("2");
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn evaluate_handles_basic_expression() {
    //     let input = vec![String::from("+"), String::from("1"), String::from("1")];
    //     let result: String = parse::evaluate(input);
    //     let expected = String::from("2");
    //     assert_eq!(result, expected);
    // }
}