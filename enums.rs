/*
Enums are a way to define a type by enumerating . They are useful for representing
a value that can be one of several different types.
* uses enum and each value is called a variant
*/

enum Directions {
    Up,
    Down,
    Right,
    Left,
}
fn main() {
    let player_direction = Directions::Up;
    match player_direction {
        Directions::Up => println!("Player is moving up"),
        Directions::Down => println!("Player is moving down"),
        Directions::Right => println!("Player is moving right"),
        Directions::Left => println!("Player is moving left"),
    }
}
