mod run;

use jasc::ast::Value;
use jasc::error::Error;

#[test]
fn number() {
    run::ok("(12);", Value::Number(12.0))
}

#[test]
fn grouping_only() {
    run::ok("(12+3);", Value::Number(15.0))
}

#[test]
fn add_mul() {
    run::ok("(12+3)*4;", Value::Number(60.0))
}

#[test]
fn add_mul_2() {
    run::ok("(12*3)+4;", Value::Number(40.0))
}

#[test]
fn mul_add() {
    run::ok("12*(3+4);", Value::Number(84.0))
}

#[test]
fn unary_minus() {
    run::ok("-(12 + 34);", Value::Number(-46.0))
}

#[test]
fn binary_minus() {
    run::ok("123-(45+67);", Value::Number(11.0))
}

#[test]
fn nested() {
    run::ok("123-(45-(67-8));", Value::Number(137.0))
}

#[test]
fn grouping_no_semicolon() {
    run::err1("(12+34)", 1, "end", "Semicolon expected.");
}

#[test]
fn missing_right_paren() {
    run::err("(12+3;", vec![Error::new(1, ";", "Right paren expected")]);
}

#[test]
fn missing_left_paren() {
    run::err("12+3);", vec![Error::new(1, ")", "Semicolon expected.")]);
}
