mod run;

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
