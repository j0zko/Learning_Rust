/*
* USER include_bytes!
* -std::io - standard user input stream I/O module
* .read_line - reads input from line
* .trim() removes white spaces
* .except() excepting crash with message on error
*
*
*
* */

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your name:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let name = input.trim();
    println!("Hello, {}", name);
}
