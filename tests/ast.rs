mod run;

#[test]
fn number() {
    run::parse("123;", "(expression 123)")
}

#[test]
fn test_bool() {
    run::parse("true;", "(expression true)");
    run::parse("false;", "(expression false)");
}

#[test]
fn unary() {
    run::parse("-123;", "(expression (- 123))");
    run::parse("-true;", "(expression (- true))");
}

#[test]
fn term() {
    run::parse("12 * -34 / 56;", "(expression (/ (* 12 (- 34)) 56))");
}

#[test]
fn factor() {
    run::parse(
        "12+-34*56-78/90;",
        "(expression (- (+ 12 (* (- 34) 56)) (/ 78 90)))",
    );
}

#[test]
fn grouping() {
    run::parse(
        "((12+-34)*56-78)/90;",
        "(expression (/ (group (- (* (group (+ 12 (- 34))) 56) 78)) 90))",
    );
    run::parse(
        "true && (-true || false);",
        "(expression (&& true (group (|| (- true) false))))",
    );
}

#[test]
fn ternary() {
    run::parse("1 ? 2 ? 3 : 4 : 5;", "(expression (? 1 (? 2 3 4) 5))");
    run::parse("1 ? 2 : 3 ? 4 : 5;", "(expression (? 1 2 (? 3 4 5)))");
    run::parse(
        "1 ? 2 ? 3 : 4 : 5 ? 6 : 7;",
        "(expression (? 1 (? 2 3 4) (? 5 6 7)))",
    );
    run::parse("true ? 1 : false;", "(expression (? true 1 false))");
}

#[test]
fn or() {
    run::parse("true || -false;", "(expression (|| true (- false)))");
    run::parse(
        "true || false && true;",
        "(expression (|| true (&& false true)))",
    );
}

#[test]
fn and() {
    run::parse("true && -false;", "(expression (&& true (- false)))");
}

#[test]
fn let_() {
    run::parse("let a = 1;", "(let a 1)");
    run::parse("let a = true;", "(let a true)");
}

#[test]
fn assignment() {
    run::parse("a = 1;", "(expression (assignment a 1))");
    run::parse(
        "a = b + c;",
        "(expression (assignment a (+ (var b) (var c))))",
    );
    run::parse("a = b = 3;", "(expression (assignment a (assignment b 3)))");
    run::parse("a = true;", "(expression (assignment a true))");
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
        "function(a, b){return a;};",
        "(expression (function (parameters a b) (statements (return (var a)))))",
    );
}

#[test]
fn if_() {
    run::parse("if (1 + 1) 2 + 2;", "(if (+ 1 1) (expression (+ 2 2)))");
    run::parse("if (1) {2;}", "(if 1 (block (expression 2)))");
    run::parse("if (1) 2; else 3;", "(if 1 (expression 2) (expression 3))");
    run::parse(
        "if (1) {2;} else {3;}",
        "(if 1 (block (expression 2)) (block (expression 3)))",
    );
}

// TODO: テスト関数・ファイルの名前を変更する
#[test]
fn test_while() {
    run::parse(
        "while (1 + 1) 2 + 2;",
        "(while (+ 1 1) (expression (+ 2 2)))",
    );
}

#[test]
fn test_break() {
    run::parse("while (1) break;", "(while 1 (break))");
    run::parse("while (1) { break; }", "(while 1 (block (break)))");
}
