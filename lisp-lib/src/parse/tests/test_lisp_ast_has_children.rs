#[cfg(test)]
mod test_lisp_ast_has_children {
    use lisp_operator::LispOperator;
    use lisp_token::LispToken;
    use parse::lisp_ast::LispAstNode;

    #[test]
    fn has_children_works() {
        let none_children = LispAstNode {
            token: LispToken::Value(String::from("1")),
            children: None,
        };
        assert_eq!(none_children.has_children(), false);

        let empty_children = LispAstNode {
            token: LispToken::Value(String::from("1")),
            children: None,
        };
        assert_eq!(empty_children.has_children(), false);

        let one_child = LispAstNode {
            token: LispToken::Operator(LispOperator::Add),
            children: Some(vec![
                LispAstNode {
                    token: LispToken::Value(String::from("1")),
                    children: None,
                },
            ]),
        };
        assert_eq!(one_child.has_children(), true);
    }
}
