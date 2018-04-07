
mod process_token;

pub fn get_tokens(input: &str) -> Vec<String> {
    let raw_input = input.to_string();
    raw_input.split_whitespace()
        .map(str::to_string)
        .flat_map(process_token::process_raw_token)
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod lex_tests {
    use lex;

    #[test]
    fn get_tokens_should_split_simple_expr_by_whitespace() {
        let input = "( + 1 1 )";
        let result = lex::get_tokens(input);
        let expected = vec![
            String::from("("),
                String::from("+"),
                    String::from("1"),
                    String::from("1"),
            String::from(")"),
        ];
        assert_eq!(result, expected);
    }

}
