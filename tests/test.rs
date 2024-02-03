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

    pub fn err1(src: &str, line: usize, location: &str, msg: &str) {
        err(src, vec![Error::new(line, location, msg)])
    }
}

mod scanner {
    use crate::run;

    #[test]
    fn unexpected_character() {
        run::err1("@", 1, "@", "Unexpected character.");
    }

    #[test]
    fn unexpected_character_at_line_2() {
        run::err1("\n@", 2, "@", "Unexpected character.");
    }
}

mod expression {

    mod number {
        use crate::run;
        use jasc::ast::Value;

        #[test]
        fn simple_number() {
            run::ok("123;", Value::Number(123.0))
        }

        #[test]
        fn number_no_semicolon() {
            run::err1("123", 1, "end", "Semicolon expected.");
        }
    }

    mod unary {
        use crate::run;
        use jasc::ast::Value;
        use jasc::error::Error;

        #[test]
        fn unary_minus() {
            run::ok("-12;", Value::Number(-12.0))
        }

        #[test]
        fn add_unary_minus() {
            run::ok("34+-12;", Value::Number(22.0))
        }

        #[test]
        fn unary_minus_missing_right() {
            run::err(
                "-;",
                vec![
                    Error::new(1, "end", "Number expected."),
                    Error::new(1, "end", "Semicolon expected."),
                ],
            );
        }
    }

    mod term {
        use crate::run;
        use jasc::ast::Value;
        use jasc::error::Error;

        #[test]
        fn add_12_34() {
            run::ok("12+34;", Value::Number(46.0))
        }

        #[test]
        fn add_12_34_56() {
            run::ok("12+34+56;", Value::Number(102.0))
        }

        #[test]
        fn sub_34_12() {
            run::ok("34-12;", Value::Number(22.0))
        }

        #[test]
        fn sub_56_34_12() {
            run::ok("56-34-12;", Value::Number(10.0))
        }

        #[test]
        fn addition_no_semicolon() {
            run::err1("12+34", 1, "end", "Semicolon expected.");
        }

        #[test]
        fn addition_no_semicolon_after_3() {
            run::err1("12+34+56", 1, "end", "Semicolon expected.");
        }

        #[test]
        fn add_missing_right() {
            run::err(
                "12+;",
                vec![
                    Error::new(1, "end", "Number expected."),
                    Error::new(1, "end", "Semicolon expected."),
                ],
            );
        }

        #[test]
        fn add_missing_right_after_2() {
            run::err(
                "12+34+;",
                vec![
                    Error::new(1, "end", "Number expected."),
                    Error::new(1, "end", "Semicolon expected."),
                ],
            );
        }
    }
}
