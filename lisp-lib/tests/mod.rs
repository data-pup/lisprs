extern crate lisp_lib;

#[cfg(test)]
mod tests {
    use lisp_lib;

    #[test]
    fn get_result() {
        let input = "(+ 1 1)";
        let result = lisp_lib::get_result(input);
        let expected = "2";
        assert_eq!(result, expected);
    }
}
