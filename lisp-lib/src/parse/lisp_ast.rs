use lisp_token::OperatorToken;

struct _Atom<T> {
    value: T
}

struct _Expression<T> {
    op: OperatorToken,
    vars: Vec<_Atom<T>>,
}

// impl Expression<T> {
//     pub fn evaluate(&self) -> Atom<T> {}
// }
