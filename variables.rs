/*
    Inmutable variables in rust
    are by default in rust ,
    they value cannot be changed once a value is bond to it

*/

fn main() {
    let employment = 500;
    println!("all of this people is employed {}", employment);

    {
        let employment = 1000;
        println!(" changed state of employed {}", employment);
    }

    let percentage: f64 = 34.00;
    println!("the percentages of unimplemented! {}", percentage);
}
