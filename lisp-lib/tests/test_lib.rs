extern crate lisp_lib;

fn run_test(test_case: &TestCase) {
    let &TestCase { input, expected } = test_case;
    let output: String = lisp_lib::get_result(input);
    assert_eq!(
        output, expected,
        "Failed Test:\tInput: '{:?}'\tExpected: '{:?}'\tOutput: '{:?}'",
        input, expected, output
    );
}

#[test]
fn basic_integer_addition_examples() {
    BASIC_INTEGER_ADDITION_TEST_CASES.iter().for_each(run_test);
}

#[test]
fn integer_addition_examples_with_nested_expressions() {
    NESTED_EXPR_INTEGER_ADDITION_TEST_CASES
        .iter()
        .for_each(run_test);
}

struct TestCase {
    input: &'static str,
    expected: &'static str,
}

static BASIC_INTEGER_ADDITION_TEST_CASES: &[TestCase] = &[
    TestCase {
        input: "+ 1",
        expected: "1",
    },
    TestCase {
        input: "( + 1 )",
        expected: "1",
    },
    TestCase {
        input: "(+ 1)",
        expected: "1",
    },
    TestCase {
        input: "( + 1 1 )",
        expected: "2",
    },
    TestCase {
        input: "(+ 1 1)",
        expected: "2",
    },
    TestCase {
        input: "((+ 1 1))",
        expected: "2",
    },
    TestCase {
        input: "(+ 1 1 1)",
        expected: "3",
    },
    TestCase {
        input: "+ 1 2",
        expected: "3",
    },
    TestCase {
        input: "(+ 10 10 10 10 10)",
        expected: "50",
    },
];

static NESTED_EXPR_INTEGER_ADDITION_TEST_CASES: &[TestCase] = &[
    TestCase {
        input: "(+ (+ 1 1) 1 1",
        expected: "4",
    },
    // TestCase { input: "(+ (+ 1 ))", expected: "1", },
    // TestCase { input: "(+ (+ (+ 1)))", expected: "3", },
];
