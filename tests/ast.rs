mod run;

#[test]
fn number() {
    run::parse("123;", "(expression 123)")
}

#[test]
fn unary() {
    run::parse("-123;", "(expression (- 123))")
}

#[test]
fn term() {
    run::parse("12 * -34 / 56;", "(expression (/ (* 12 (- 34)) 56))")
}

#[test]
fn factor() {
    run::parse(
        "12+-34*56-78/90;",
        "(expression (- (+ 12 (* (- 34) 56)) (/ 78 90)))",
    )
}

#[test]
fn grouping() {
    run::parse(
        "((12+-34)*56-78)/90;",
        "(expression (/ (group (- (* (group (+ 12 (- 34))) 56) 78)) 90))",
    )
}

#[test]
fn ternary() {
    run::parse("1 ? 2 ? 3 : 4 : 5;", "(expression (? 1 (? 2 3 4) 5))");
    run::parse("1 ? 2 : 3 ? 4 : 5;", "(expression (? 1 2 (? 3 4 5)))");
    run::parse(
        "1 ? 2 ? 3 : 4 : 5 ? 6 : 7;",
        "(expression (? 1 (? 2 3 4) (? 5 6 7)))",
    );
}

#[test]
fn let_() {
    run::parse("let a = 1;", "(let a 1)");
}

#[test]
fn assignment() {
    run::parse("a = 1;", "(expression (assignment a 1))");
    run::parse(
        "a = b + c;",
        "(expression (assignment a (+ (var b) (var c))))",
    );
    run::parse("a = b = 3;", "(expression (assignment a (assignment b 3)))");
}

#[test]
fn block() {
    run::parse("{}", "(block )");
    run::parse("{123;456;}", "(block (expression 123) (expression 456))");
    run::parse(
        "{123;{456;}}",
        "(block (expression 123) (block (expression 456)))",
    );
}

#[test]
fn function() {
    run::parse(
        "function(){};",
        "(expression (function (parameters) (statements)))",
    );
    run::parse(
        "function(a){print a;};",
        "(expression (function (parameters a) (statements (print (var a)))))",
    );
    run::parse(
        "function(a, b){print a;};",
        "(expression (function (parameters a b) (statements (print (var a)))))",
    );
}

#[test]
fn if_() {
    run::parse("if (1) {2;}", "(if (1) (block (expression 2)) (None))");
}
