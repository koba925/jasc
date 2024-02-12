mod run;

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
fn add_unary_minus() {
    run::ok("34+-12;", Value::Number(22.0))
}

#[test]
fn add_factors() {
    run::ok("1*-2*3+4*5/2;", Value::Number(4.0))
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
    run::err1("12+;", 1, ";", "Exrpression expected, found `;`");
}

#[test]
fn add_missing_right_2_lines() {
    run::err(
        "12+34+;
        45+67+;",
        vec![
            Error::new(1, ";", "Exrpression expected, found `;`"),
            Error::new(2, ";", "Exrpression expected, found `;`"),
        ],
    );
}
