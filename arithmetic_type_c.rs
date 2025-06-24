/*
*   ARITHMETIC and TYPE CASTING
*   +
*   -
*   *
*   /
*   % modulus
*   ------------------------
*   NOTE for a datatype i can assign a higher value , it can cause overflow
*   -------------------------
*   TYPE CASTING
*   literars as a specific type var_name= 65_i32;
*
* */
fn main() {
    // Arithmetic operations
    let a = 10_i32;
    let b = 3_i32;

    let sum = a + b; // Addition
    let difference = a - b; // Subtraction
    let product = a * b; // Multiplication
    let quotient = a / b; // Division
    let remainder = a % b; // Modulus

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);

    // Type casting
    let x = 65_i32;
    let y: f64 = x as f64; // Casting i32 to f64
    println!("Type casted value: {}", y);
}
