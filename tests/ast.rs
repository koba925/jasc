mod run;

#[test]
fn number() {
    run::parse("123;", "(expression 123)\n")
}

#[test]
fn unary() {
    run::parse("-123;", "(expression (- 123))\n")
}

#[test]
fn term() {
    run::parse("12 * -34 / 56;", "(expression (/ (* 12 (- 34)) 56))\n")
}

#[test]
fn factor() {
    run::parse(
        "12+-34*56-78/90;",
        "(expression (- (+ 12 (* (- 34) 56)) (/ 78 90)))\n",
    )
}

#[test]
fn grouping() {
    run::parse(
        "((12+-34)*56-78)/90;",
        "(expression (/ (group (- (* (group (+ 12 (- 34))) 56) 78)) 90))\n",
    )
}

#[test]
fn ternary() {
    run::parse("1 ? 2 ? 3 : 4 : 5;", "(expression (? 1 (? 2 3 4) 5))\n");
    run::parse("1 ? 2 : 3 ? 4 : 5;", "(expression (? 1 2 (? 3 4 5)))\n");
    run::parse(
        "1 ? 2 ? 3 : 4 : 5 ? 6 : 7;",
        "(expression (? 1 (? 2 3 4) (? 5 6 7)))\n",
    );
}

#[test]
fn let_() {
    run::parse("let a = 1;", "(let a 1)\n");
}

#[test]
fn assignment() {
    run::parse("a = 1;", "(expression (assignment a 1))\n");
    run::parse(
        "a = b + c;",
        "(expression (assignment a (+ (var b) (var c))))\n",
    );
    run::parse(
        "a = b = 3;",
        "(expression (assignment a (assignment b 3)))\n",
    );
}
