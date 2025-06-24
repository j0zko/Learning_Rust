/*
*   FUNCTIONS
*   functions are reusable blocks of code and can be used many times
*
*
* */

fn main() {
    //calling the function hello
    hello();
    println!("returning { }", returning_function());
}

fn hello() {
    println!("hello");
}

fn returning_function() -> i32 {
    let a = 20;
    let b = 30;
    a + b
}
