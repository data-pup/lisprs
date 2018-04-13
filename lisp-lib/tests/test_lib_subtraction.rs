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
fn basic_integer_subtraction_works() {
    BASIC_INTEGER_SUBTRACTION_TEST.iter().for_each(run_test);
}

struct TestCase {
    input: &'static str,
    expected: &'static str,
}

static BASIC_INTEGER_SUBTRACTION_TEST: &[TestCase] = &[
    TestCase {
        input: "- 1",
        expected: "-1",
    },
    TestCase {
        input: "- 1 1",
        expected: "0",
    },
    TestCase {
        input: "- 96 20 1 1 1",
        expected: "73",
    },
];
