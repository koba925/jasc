use jasc::ast::Value;
use jasc::error::Error;

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
        Ok(expr) => assert_eq!(format!("{}", expr), expected),
        _ => panic!("Failed - result: {:?}", result),
    }
}
