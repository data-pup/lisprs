mod eval_error;
use self::eval_error::EvalError;

use parse::LispAstNode;
use lisp_token::LispToken::{
    // Operator,
    Variable,
    Value,
};

pub fn evaluate(ast: LispAstNode) -> String {
    match evaluate_helper(ast) {
        Ok(_) => unimplemented!(),
        Err(_) => String::from("Error!"),
    }
}

fn evaluate_helper(ast: LispAstNode) -> Result<f64, EvalError> {
    match ast.has_children() {
        true => evaluate_expr(ast),
        false => evaluate_leaf_node(ast),
    }
}

fn evaluate_expr(_node: LispAstNode) -> Result<f64, EvalError> {
    unimplemented!();
}

fn evaluate_leaf_node(node: LispAstNode) -> Result<f64, EvalError> {
    match node.token {
        Variable(_) => unimplemented!(),
        Value(val_str) => {
            let val = val_str.parse::<f64>();
            if val.is_ok() { Ok(val.unwrap())                    }
            else           { Err(EvalError::ValueParseError) }
        },
        _ => Err(EvalError::InvalidLeafNode),
    }
}

#[cfg(test)]
mod evaluate_leaf_node_tests {
    use evaluate::{evaluate_leaf_node, EvalError};
    use lisp_token::LispToken;
    use parse::LispAstNode;

    #[test]
    fn leaf_node_that_is_not_var_or_val_causes_error() {
        unimplemented!();
    }

    struct ValueParseTestCase {
        input: &'static str,
        expected: Result<f64, EvalError>,
        desc: &'static str,
    }

    static PARSE_TESTS: &[ValueParseTestCase] = &[
        ValueParseTestCase {
            input: "1", expected: Ok(1_f64), desc: "Parse 1",
        },
        ValueParseTestCase {
            input: "100", expected: Ok(100_f64), desc: "Parse 100",
        },
        ValueParseTestCase {
            input: "1.5", expected: Ok(1.5_f64), desc: "Parse 1.5",
        },
        ValueParseTestCase {
            input: "-1", expected: Ok(-1_f64), desc: "Parse -1",
        },
        ValueParseTestCase {
            input: "-0.005", expected: Ok(-0.005_f64), desc: "Parse -0.005",
        },
    ];

    #[test]
    fn value_node_can_be_parsed_to_f64() {
        for test_case in PARSE_TESTS.iter() {
            let &ValueParseTestCase { input, ref expected, desc } = test_case;
            let node = LispAstNode {
                token: LispToken::Value(String::from(input)),
                children: None,
            };
            let result = evaluate_leaf_node(node);
            if expected.is_ok() {
                let expected_result = expected.as_ref().unwrap();
                let actual_result = result.unwrap();
                assert_eq!(actual_result, *expected_result,
                    "Test Failed: {}", desc);
            } else {
                let expected_err = expected.as_ref().unwrap_err();
                let actual_err = result.unwrap_err();
                assert_eq!(actual_err, *expected_err, "Test Failed: {}", desc);
            }
        }
    }
}
