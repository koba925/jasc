use jasc::ast;
use jasc::ast::Value;
use jasc::error::Error;
use jasc::token;

#[allow(dead_code)]
pub fn ok(src: &str, expected: Value) {
    let result = jasc::run(src);
    match result {
        Ok(value) => assert_eq!(value, expected),
        _ => panic!("Failed - result: {:?}", result),
    }
}

#[allow(dead_code)]
pub fn err(src: &str, vexpected: Vec<Error>) {
    let result = jasc::run(src);
    match result {
        Err(ve) => assert_eq!(ve, vexpected),
        _ => panic!("Failed - result: {:?}", result),
    };
}

#[allow(dead_code)]
pub fn err1(src: &str, line: usize, location: &str, msg: &str) {
    err(src, vec![Error::new(line, location, msg)])
}

#[allow(dead_code)]
pub fn parse(src: &str, expected: &str) {
    let result = jasc::parse(src);
    match result {
        Ok(stmts) => assert_eq!(ast::stringify_statements(&stmts), expected),
        _ => panic!("Failed - result: {:?}", result),
    }
}

#[allow(dead_code)]
pub fn scan(src: &str, expected: &str) {
    let result = jasc::scan(src);
    match result {
        Ok(tokens) => assert_eq!(token::stringify_tokens(&tokens), expected),
        _ => panic!("Failed - result: {:?}", result),
    }
}

#[allow(dead_code)]
pub fn scan_err(src: &str, vexpected: Vec<Error>) {
    let result = jasc::scan(src);
    match result {
        Err(ve) => assert_eq!(ve, vexpected),
        _ => panic!("Failed - result: {:?}", result),
    };
}

#[allow(dead_code)]
pub fn scan_err1(src: &str, line: usize, location: &str, msg: &str) {
    scan_err(src, vec![Error::new(line, location, msg)])
}
