/*
*    TUPLE
*   allows to store multiple data with different data_types
*   constructed using () parenthes
*   can be accessed by positions - tuple indexing
*
*
*
* */

fn main() {
    let tup: (i32, char, bool) = (1, 'b', true);

    println!("Debug format: {:?}", tup);
    println!("Accessin the last element in tup : {}", tup.2);
    //Destructure
    let (num, letter, flag) = tup;
    println!("Destructured: {} - {} - {}", num, letter, flag);
}
