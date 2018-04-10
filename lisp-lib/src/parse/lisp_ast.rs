use lisp_operator::LispOperator;

#[derive(Debug, PartialEq)]
struct _Atom { value: i64 }

impl _Atom {
    pub fn new() -> Self { Self { value: 0_i64 } }
}

struct _Expression {
    op: LispOperator,
    vars: Vec<_Atom>,
}

impl _Expression {
    pub fn evaluate(&self) -> _Atom {
        match self.op {
            LispOperator::Add => {
                self.vars.iter().fold(_Atom::new(),
                    |res, acc| _Atom { value: res.value + acc.value }
                )
            },
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod evaluate_tests {
    use parse::lisp_ast::{_Atom, _Expression};
    use lisp_operator::LispOperator;

    #[test]
    fn expression_evaluates_one_plus_two() {
        let expr = _Expression {
            op: LispOperator::Add,
            vars: vec![
                _Atom { value: 1_i64 },
                _Atom { value: 2_i64 },
            ],
        };
        let expected = _Atom { value: 3_i64 };
        let result = expr.evaluate();
        assert_eq!(result, expected);
    }
}
