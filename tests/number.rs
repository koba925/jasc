use jasc::error::Error;

fn run_ok(src: &str, expected: f64) {
    match jasc::run(src.to_string()) {
        Ok(value) => assert_eq!(value, expected),
        Err(ve) => panic!("Failed - ve: {:?}", ve),
    }
}

fn run_err(src: &str, vexpected: Vec<Error>) {
    let result = jasc::run(src.to_string());
    match result {
        Err(ve) => {
            assert_eq!(ve, vexpected)
        }
        _ => panic!("Failed - result: {:?}", result),
    };
}

#[test]
fn unexpected_character() {
    run_err(
        "@",
        vec![Error::new(1, "Unexpected character ('@').".to_string())],
    );
}

#[test]
fn simple_number() {
    run_ok("123;", 123.0)
}

#[test]
fn no_semicolon() {
    run_err(
        "123",
        vec![Error::new(1, "Semicolon expected.".to_string())],
    );
}
