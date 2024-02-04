mod run;

#[test]
fn unexpected_character() {
    run::err1("@", 1, "@", "Unexpected character.");
}

#[test]
fn unexpected_character_at_line_2() {
    run::err1("\n@", 2, "@", "Unexpected character.");
}
