use lisp_operator::OperatorToken;

#[derive(Debug, PartialEq)]
pub enum LispToken {
    OpenExpression,
    Operator(OperatorToken),
    Variable(String),
    Value(String),
    CloseExpression,
}
