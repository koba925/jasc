mod run;

use jasc::ast::Value;

#[test]
fn mul_12_3() {
    run::ok("12*3;", Value::Number(36.0))
}

#[test]
fn mul_2_3_4() {
    run::ok("2*3*4;", Value::Number(24.0))
}

#[test]
fn mul_unary_minus() {
    run::ok("12*-3;", Value::Number(-36.0))
}

#[test]
fn div_12_3() {
    run::ok("12/3;", Value::Number(4.0))
}

#[test]
fn div_12_3_4() {
    run::ok("12/3/4;", Value::Number(1.0))
}

#[test]
fn mul_no_semicolon() {
    run::err1("12*34", 1, "end", "Semicolon expected.");
}

#[test]
fn mul_no_semicolon_after_3() {
    run::err1("12*34*56", 1, "end", "Semicolon expected.");
}

#[test]
fn mul_missing_right() {
    run::err1("12*;", 1, ";", "Expression expected, found `;`");
}
