use lisp_token::LispToken;

mod lisp_ast;
mod lisp_ast_from_tokens;
mod parse_errors;
mod tests;

pub use self::parse_errors::_ParseError;
pub use self::lisp_ast::_LispAstNode;

// Parse a vector of tokens, evaluate the expression, and return the result.
pub fn parse(_tokens: Vec<LispToken>) -> Result<_LispAstNode, _ParseError> {
    unimplemented!();
}
