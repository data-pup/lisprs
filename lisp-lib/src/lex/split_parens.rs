use std::iter::FromIterator;

use lex::token_regex::is_paren;

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
