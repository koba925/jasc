use jasc::ast::Value;
use jasc::error::Error;

fn run_ok(src: &str, expected: Value) {
    match jasc::run(src) {
        Ok(value) => assert_eq!(value, expected),
        Err(ve) => panic!("Failed - ve: {:?}", ve),
    }
}

fn run_err(src: &str, vexpected: Vec<Error>) {
    let result = jasc::run(src);
    match result {
        Err(ve) => {
            assert_eq!(ve, vexpected)
        }
        _ => panic!("Failed - result: {:?}", result),
    };
}

#[test]
fn unexpected_character() {
    run_err("@", vec![Error::new(1, "Unexpected character ('@').")]);
}

#[test]
fn simple_number() {
    run_ok("123;", Value::Number(123.0))
}

#[test]
fn no_semicolon() {
    run_err("123", vec![Error::new(1, "Semicolon expected.")]);
}

#[test]
fn simple_addition() {
    run_ok("12+34;", Value::Number(46.0))
}
