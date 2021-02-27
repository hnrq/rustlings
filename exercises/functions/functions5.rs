// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// If the return statement is not explicitly written, the last expression can't have a semicolon

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
