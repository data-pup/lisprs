pub enum _LispToken {
    OpenExpression,
    Operator(_OperatorToken),
    Token(String),
    CloseExpression,
}

pub enum _OperatorToken {
    Add,
    Subtract,
    Multiply,
    Divide,
}
