use jasc::ast::Value;

mod run;

#[test]
fn unexpected_character() {
    run::err1("@", 1, "@", "Unexpected character.");
}

#[test]
fn unexpected_character_at_line_2() {
    run::err1("\n@", 2, "@", "Unexpected character.");
}

#[test]
fn comment() {
    run::ok("12 + 34; // comment", Value::Number(46.0));
    run::ok("12 + // comment\n 34; ", Value::Number(46.0));
    run::err1("12 + // comment\n 34", 2, "end", "Semicolon expected.");
}
