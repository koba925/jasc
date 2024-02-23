use jasc::ast::Value;
use std::io;

fn main() {
    let src = io::read_to_string(io::stdin()).expect("Error: failed to read the code.");
    match jasc::run(src) {
        Ok(Value::Null) => {}
        Ok(value) => println!("{}", value),
        Err(errors) => {
            for e in errors {
                e.report();
            }
        }
    }
}
