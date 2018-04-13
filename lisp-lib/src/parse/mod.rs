mod lisp_ast;
mod lisp_ast_from_tokens;
mod parse_errors;
mod tests;

use lisp_token::LispToken;
use std::convert::TryFrom;

pub use self::lisp_ast::LispAstNode;
pub use self::parse_errors::_ParseError;

// Parse a vector of tokens, evaluate the expression, and return the result.
pub fn parse(_tokens: Vec<LispToken>) -> Result<LispAstNode, _ParseError> {
    LispAstNode::try_from(_tokens)
}
