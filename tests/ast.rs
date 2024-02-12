mod run;

#[test]
fn number() {
    run::parse("123;", "123")
}

#[test]
fn unary() {
    run::parse("-123;", "(- 123)")
}

#[test]
fn term() {
    run::parse("12 * -34 / 56;", "(/ (* 12 (- 34)) 56)")
}

#[test]
fn factor() {
    run::parse("12+-34*56-78/90;", "(- (+ 12 (* (- 34) 56)) (/ 78 90))")
}

#[test]
fn grouping() {
    run::parse(
        "((12+-34)*56-78)/90;",
        "(/ (group (- (* (group (+ 12 (- 34))) 56) 78)) 90)",
    )
}
