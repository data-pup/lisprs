#[derive(Debug, PartialEq)]
pub enum _ParseError {
    EmptyExpression,
    UnexpectedParen,
    UnexpectedToken,
    MissingOperands,
    InvalidSyntaxTree,
}
