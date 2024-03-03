mod run;
use jasc::error::Error;

#[test]
fn number() {
    run::scan("123", "(Number(123.0) '123' 1)\n(EOF '' 1)\n");
}

#[test]
fn parens() {
    run::scan("()", "(LeftParen '(' 1)\n(RightParen ')' 1)\n(EOF '' 1)\n");
}

#[test]
fn braces() {
    run::scan("{}", "(LeftBrace '{' 1)\n(RightBrace '}' 1)\n(EOF '' 1)\n");
}

#[test]
fn skip_whitespaces() {
    run::scan(
        " \t\n(  \t\t\n\n) \t\n",
        "(LeftParen '(' 2)\n(RightParen ')' 4)\n(EOF '' 5)\n",
    );
}

#[test]
fn unexpected_charcter() {
    run::scan_err1("@", 1, "@", "Unexpected character.");
}

#[test]
fn multi_errors_multi_lines() {
    run::scan_err(
        "@\n^)",
        vec![
            Error::new(1, "@", "Unexpected character."),
            Error::new(2, "^", "Unexpected character."),
        ],
    );
}
