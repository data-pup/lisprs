#[derive(Debug, PartialEq)]
pub enum _ParseError {
    EmptyExpression,
    InvalidSyntaxTree,
    MissingOperands,
    UnexpectedParen,
}
