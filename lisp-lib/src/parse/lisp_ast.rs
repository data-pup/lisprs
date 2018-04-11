use lisp_token::LispToken;

#[derive(Debug, PartialEq)]
pub struct LispAstNode {
    pub token: LispToken,
    pub children: Option<Vec<LispAstNode>>,
}

impl LispAstNode {
    pub fn has_children(&self) -> bool {
        if self.children.is_none() { false }
        else if self.children.as_ref().unwrap().is_empty() { false }
        else { true }
    }
}
