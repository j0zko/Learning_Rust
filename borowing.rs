fn main() {
    let s = String::from("Hello");
    print_lenght(&s);
    println!("{}", s1);
}
fn print_lenght(s: &String) {
    println!("lenght {}", s.len());
}
