use lisp_token::LispToken;

#[derive(Debug, PartialEq)]
pub struct LispAstNode {
    pub token: LispToken,
    pub children: Option<Vec<LispAstNode>>,
}
