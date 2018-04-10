use lisp_token::LispToken;

#[derive(Debug, PartialEq)]
pub struct _LispAstNode {
    pub token: LispToken,
    pub children: Option<Vec<_LispAstNode>>,
}
