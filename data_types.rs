/*
*   DATATYPES IN RUST
*   DATATYPE - A datatype defines data what a var can hold
*
*   Types Primitive,Scalar,Compound
*   Primitive(INT) signed and unsigned(cannot assign negative)
*   i8 ---u8
*   i16---u16
*   i32 ---u32
*   i64 ---u64
*   i128 ---u128
* --------------------
* Floating point
*  f32 and f64 -single and double precision
* -------------------
* Charakter -char using ''
*   represnts a single charakter - '-'
* ----------------------------------
* Booleans represents a value , which could be true or false
*
* */

fn main() {
    let mut value: f32 = 10.55;
    println!("this is a value {}", value);
    value = 44.55;
    println!("This value was changed {}", value);
    data1();
    data_char();
}

fn data1() {
    let true_or_false: bool = false;
    println!("this is a boolean {}", true_or_false);
}
fn data_char() {
    let charakter: char = 'R';
    println!("this is a charakter {}"charakter);
}
