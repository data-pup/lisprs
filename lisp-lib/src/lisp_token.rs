use lisp_operator::LispOperator;

#[derive(Debug, PartialEq)]
pub enum LispToken {
    OpenExpression,
    Operator(LispOperator),
    Variable(String),
    Value(String),
    CloseExpression,
}
