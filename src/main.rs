// TODO:matchesを作る → 必要か？

use std::io;

fn main() {
    let src = io::read_to_string(io::stdin()).expect("Error: failed to read the code.");
    match jasc::run(src) {
        Ok(value) => println!("{:?}", value),
        Err(errors) => {
            for e in errors {
                e.report();
            }
        }
    }
}
