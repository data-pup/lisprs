use std::iter::FromIterator;
use lex::token_regex::{
    is_syntax,
    is_value,
    is_variable,
};

/// Split a raw token into its syntactically meaningful components.
pub fn process_raw_token(token: String) -> Vec<String> {
    match &token {
        t if is_variable(&t)  // Return the token as is if it is a variable,
          || is_value(&t)     // value token, or syntactivally relevant.
          || is_syntax(&t) => vec![token.clone()],
        t => { // Otherwise, process the components of the token.
            let mut tokens: Vec<String> = vec![];
            let mut curr_token: Vec<char> = vec![];
            for curr_c in t.chars() {
                match curr_c {
                    c if is_syntax(&c.to_string()) => {
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

#[cfg(test)]
mod raw_token_processing_tests {
    use lex::process_token;

    #[test]
    fn basic_tokens_should_be_returned_identically() {
        for input in ["hello", "+", "(", "0"].into_iter() {
            let input_s = input.to_string();
            let expected = vec![input_s.clone()];
            let result = process_token::process_raw_token(input_s.clone());
            assert_eq!(result, expected, "Incorrectly split: {}", input_s);
        }
    }

    #[test]
    fn token_with_paren_and_op_should_be_split() {
        let input = String::from("(+");
        let result = process_token::process_raw_token(input);
        let expected = vec![String::from("("), String::from("+")];
        assert_eq!(result, expected, "Did not split parentheses with operator.");
    }

    #[test]
    fn value_wrapped_in_parens_should_be_split() {
        let input = String::from("(1)");
        let result = process_token::process_raw_token(input);
        let expected = vec![String::from("("), String::from("1"), String::from(")")];
        assert_eq!(result, expected, "Did not split parentheses with operator.");
    }
}
