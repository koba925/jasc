mod run {
    use jasc::ast::Value;
    use jasc::error::Error;

    pub fn ok(src: &str, expected: Value) {
        let result = jasc::run(src);
        match result {
            Ok(value) => assert_eq!(value, expected),
            _ => panic!("Failed - result: {:?}", result),
        }
    }

    pub fn err(src: &str, vexpected: Vec<Error>) {
        let result = jasc::run(src);
        match result {
            Err(ve) => assert_eq!(ve, vexpected),
            _ => panic!("Failed - result: {:?}", result),
        };
    }

    pub fn err1(src: &str, line: usize, msg: &str) {
        err(src, vec![Error::new(line, msg)])
    }
}

mod scanner {
    use crate::run;

    #[test]
    fn unexpected_character() {
        run::err1("@", 1, "Unexpected character ('@').");
    }
}

mod expression {
    use crate::run;
    use jasc::ast::Value;

    #[test]
    fn simple_number() {
        run::ok("123;", Value::Number(123.0))
    }

    #[test]
    fn simple_addition() {
        run::ok("12+34;", Value::Number(46.0))
    }

    #[test]
    fn addition_missing_right() {
        run::err1("12+;", 1, "Number expected.");
    }

    #[test]
    fn number_no_semicolon() {
        run::err1("123", 1, "Semicolon expected.");
    }

    #[test]
    fn addition_no_semicolon() {
        run::err1("12+34", 1, "Semicolon expected.");
    }
}
