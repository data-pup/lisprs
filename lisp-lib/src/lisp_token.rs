#[derive(Debug, PartialEq)]
pub enum LispToken {
    OpenExpression,
    Operator(OperatorToken),
    Variable(String),
    Value(String),
    CloseExpression,
}

#[derive(Debug, PartialEq)]
pub enum OperatorToken {
    Add,
    Subtract,
    Multiply,
    Divide,
}
