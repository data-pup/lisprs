use regex::Regex;

lazy_static! {
    static ref VAR_TOKEN:  Regex = Regex::new(r"[A-Za-z]").unwrap();
    static ref PAREN_CHAR: Regex = Regex::new(r"^[()]$").unwrap();
    static ref OP_CHAR:    Regex = Regex::new(r"^[+\-*/]$").unwrap();
}

pub fn get_tokens(input: &str) -> Vec<String> {
    let raw_input = input.to_string();
    raw_input.split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod lex_tests {
    use lex;
    use regex;

    #[test]
    fn get_tokens_should_split_by_whitespace() {
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

    #[test]
    fn regexes_should_identify_parens() {
        for curr in ["(", ")"].into_iter() {
            let is_match = lex::PAREN_CHAR.is_match(curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ["a", "[", "1"].into_iter() {
            let is_match = lex::PAREN_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }

        for curr in ["(+", "hello)"].into_iter() {
            let is_match = lex::PAREN_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }

    #[test]
    fn regexes_should_match_operators() {
        for curr in ["+", "-", "*", "/",].into_iter() {
            let is_match = lex::OP_CHAR.is_match(curr);
            assert_eq!(is_match, true, "Did not correctly match: {}", curr);
        }

        for curr in ["!", ",", ".", "?"].into_iter() {
            let is_match = lex::OP_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }

        for curr in ["++", "/hello", "hello/"].into_iter() {
            let is_match = lex::OP_CHAR.is_match(curr);
            assert_eq!(is_match, false, "Incorrectly matched: {}", curr);
        }
    }
}
