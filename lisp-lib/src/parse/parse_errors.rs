#[derive(Debug, PartialEq)]
pub enum _ParseError {
    EmptyExpression,
    MissingOperands,
    InvalidSyntaxTree,

    // Parentheses related parse errors.
    UnexpectedParen,
    UnexpectedToken,
    UnexpectedOpenParen,
}
