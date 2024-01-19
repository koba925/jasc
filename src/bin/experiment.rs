#[derive(Debug)]
enum MyError {
    ME1 { line: usize, msg: String },
}

fn run() -> Result<i32, Vec<MyError>> {
    let mut ve = vec![];
    let e1 = MyError::ME1 {
        line: 10,
        msg: "Error occurred.".to_string(),
    };
    ve.push(e1);
    Err(ve)
}

fn main() {
    let result = run();
    match result {
        Ok(value) => println!("{value}"),
        Err(ve) => {
            for e in ve {
                match e {
                    MyError::ME1 { line, msg } => println!("line {}: {}", line, msg),
                }
            }
        }
    }
}

#[test]
fn test_run() {
    let result = run();
    match result {
        Ok(value) => assert_eq!(value, 10),
        Err(ve) => {
            for e in ve {
                match e {
                    MyError::ME1 { line, msg } => {
                        assert_eq!(line, 10);
                        assert_eq!(msg, "Error occurred.");
                    }
                }
            }
        }
    }
}
