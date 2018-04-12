#[derive(Debug, PartialEq)]
pub enum EvalError {
    InvalidLeafNode,
    InvalidOp,
    MissingOperands,
    ValueParseError,
}
