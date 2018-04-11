#[cfg(test)]
mod op_processing_tests {
    use lex::process_op_token::process_op_token;
    use lisp_operator::LispOperator;
    use lisp_token::LispToken;

    #[test]
    fn op_processing_works() {
        let add_token = String::from("+"); // Check that "+" works.
        let add_op = process_op_token(&add_token).unwrap();
        assert_eq!(add_op, LispToken::Operator(LispOperator::Add), "Failed + test.");

        let sub_token = String::from("-"); // Check that "-" works.
        let sub_op = process_op_token(&sub_token).unwrap();
        assert_eq!(sub_op, LispToken::Operator(LispOperator::Subtract), "Failed - test.");

        let mult_token = String::from("*"); // Check that "*" works.
        let mult_op = process_op_token(&mult_token).unwrap();
        assert_eq!(mult_op, LispToken::Operator(LispOperator::Multiply), "Failed * test.");

        let div_token = String::from("/"); // Check that "/" works.
        let div_op = process_op_token(&div_token).unwrap();
        assert_eq!(div_op, LispToken::Operator(LispOperator::Divide), "Failed / test.");

        let invalid_op_token = String::from("$");
        let div_op = process_op_token(&invalid_op_token);
        assert!(div_op.is_none());
    }
}
