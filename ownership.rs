//*
//  OWNERSHIP IN RUST
//  Each value in rust has a variable that is called OWNER of the value
//  every data will have an owner associated with it -let age = 30
//
//  --each data can have only one owner at a time
//  --two variables cannot point to the same mem location , var will alws pointing to diff mem
//
//

fn main() {
    let mut s = String::from("Hello");

    s.push_str(",world");
    println!("{s}");
}
