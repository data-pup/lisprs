use lisp_value::LispValue;

pub struct _LispVariable {
    pub id: String,
    pub val: LispValue,
}

// impl _LispVariable {
//     pub fn new(id_s: &str, val_s: &str) -> Self {
//         unimplemented!();
//     }
// }

#[cfg(test)]
mod lisp_variable_tests {
    use lisp_variable::*;

    #[test]
    fn placeholder() {
        assert_eq!(1, 2, "Variables Tests Not Implemented!");
    }
}
