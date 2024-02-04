mod run;

use jasc::ast::Value;

#[test]
fn simple_number() {
    run::ok("123;", Value::Number(123.0))
}

#[test]
fn number_no_semicolon() {
    run::err1("123", 1, "end", "Semicolon expected.");
}
