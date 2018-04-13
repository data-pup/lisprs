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
fn basic_integer_multiplication_works() {
    BASIC_INTEGER_MULTIPLICATION_TESTS.iter().for_each(run_test);
}

struct TestCase {
    input: &'static str,
    expected: &'static str,
}

static BASIC_INTEGER_MULTIPLICATION_TESTS: &[TestCase] = &[
    TestCase {
        input: "* 1",
        expected: "1",
    },
    TestCase {
        input: "* 1 2",
        expected: "2",
    },
    TestCase {
        input: "* 10 10",
        expected: "100",
    },
    TestCase {
        input: "(* 3 3)",
        expected: "9",
    },
    TestCase {
        input: "(* 3 (* 2 3))",
        expected: "18",
    },
];
