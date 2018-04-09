enum _ParseError {
    EmptyExpression,
    MissingOpenParen,
}

// Parse a vector of tokens, evaluate the expression, and return the result.
pub fn parse(mut tokens: Vec<String>) -> String {
    let _expr_end = tokens.pop();
    let mut token_iter = tokens.into_iter();
    let _expr_start = token_iter.next().unwrap();
    let args = token_iter.collect::<Vec<String>>();
    evaluate(args)
}

/// Evaluate an expression given as a vector of strings.
fn evaluate(args: Vec<String>) -> String {
    let mut args_iter = args.into_iter();
    if let Some(op) = args_iter.next() {
        let result_val: i64 = match op.as_ref() {
            "+" => {
                args_iter.fold(0, |res: i64, next_arg| -> i64 {
                    res + next_arg.parse::<i64>().unwrap()
                })
            },
            _ => unimplemented!(),
        };
        return result_val.to_string();
    }
    unimplemented!();
}

#[cfg(test)]
mod parse_tests {
    use parse;

    #[test]
    fn parse_handles_basic_expression() {
        let input = vec![
            String::from("("),
                String::from("+"), String::from("1"), String::from("1"),
            String::from(")")
        ];
        let result: String = parse::parse(input);
        let expected = String::from("2");
        assert_eq!(result, expected);
    }

    #[test]
    fn evaluate_handles_basic_expression() {
        let input = vec![String::from("+"), String::from("1"), String::from("1")];
        let result: String = parse::evaluate(input);
        let expected = String::from("2");
        assert_eq!(result, expected);
    }
}