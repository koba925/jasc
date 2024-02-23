mod run;

use jasc::ast::Value;

#[test]
fn unary_minus() {
    run::ok("-12;", Value::Number(-12.0))
}

#[test]
fn unary_minus_missing_right() {
    run::err1("-;", 1, ";", "Expression expected, found `;`");
}
