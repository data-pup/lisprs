#[cfg(test)]
mod split_parens_tests {
    use lex::split_parens;
    #[test]
    fn split_parens_works() {
        let test_cases = vec![
            ("(", vec![String::from("(")]),
            ("(+", vec![String::from("("), String::from("+")]),
            ("+)", vec![String::from("+"), String::from(")")]),
            ("hello)", vec![String::from("hello"), String::from(")")]),
            (
                "(hello)",
                vec![String::from("("), String::from("hello"), String::from(")")],
            ),
        ];
        for (input, expected) in test_cases.into_iter() {
            let output = split_parens::split_paren_token(input);
            assert_eq!(output, expected, "Test Failed: {}", input);
        }
    }
}
