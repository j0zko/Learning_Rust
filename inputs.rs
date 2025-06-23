fn main() {
    let mut input = String::new();
    println!("Enter something:");
    let b1 = std::io::stdin().read_line(&mut input).unwrap();
    println!("Hello, {}", input);
    println!("hi, {}", b1);
}
