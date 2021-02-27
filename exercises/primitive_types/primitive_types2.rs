// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)


fn main() {
    // Characters (`char`)

    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let my_last_initial = 'R';
    if my_last_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_last_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
