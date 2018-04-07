extern crate lisp_lib;

#[cfg(test)]
mod tests {
    use lisp_lib::hello;

    #[test]
    fn it_works() {
        let s = hello();
        assert_eq!(s, "Hello World");
    }
}
