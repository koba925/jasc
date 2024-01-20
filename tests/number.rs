#[test]
fn unexpected_character() {
    let result = jasc::run("@".to_string());
    match result {
        Err(ve) if ve.len() == 1 => match ve.get(0) {
            Some(jasc::error::Error { line, msg }) => {
                assert_eq!(*line, 1);
                assert_eq!(*msg, "Unexpected character ('@').");
            }
            _ => panic!("Failed - ve: {:?}", ve),
        },
        _ => panic!("Failed - result: {:?}", result),
    };
}

#[test]
fn simple_number() {
    match jasc::run("123;".to_string()) {
        Ok(value) => assert_eq!(value, 123.0),
        _ => panic!("failed"),
    }
}

#[test]
fn no_semicolon() {
    let result = jasc::run("123".to_string());
    match result {
        Err(ve) if ve.len() == 1 => match ve.get(0) {
            Some(jasc::error::Error { line, msg }) => {
                assert_eq!(*line, 1);
                assert_eq!(*msg, "Semicolon expected.");
            }
            _ => panic!("Failed - ve: {:?}", ve),
        },
        _ => panic!("Failed - result: {:?}", result),
    };
}
