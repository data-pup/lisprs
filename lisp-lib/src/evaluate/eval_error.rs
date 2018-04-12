#[derive(Debug, PartialEq)]
pub enum EvalError {
    InvalidLeafNode,
    ValueParseError,
}
