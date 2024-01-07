use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).expect("Error: failed to read the code.");
    let src: Vec<char> = buf.chars().collect();

    if src.len() < 2 {
        panic!("Error: unexpected end of file.");
    }
    let num = src[0].to_digit(10).expect("Error: number expected.");
    if src[1] != ';' {
        panic!("Error: semicolon expected.")
    }

    println!("{num}");
}
