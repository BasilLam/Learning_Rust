use std::io;

fn main() {
    println!("Please enter a value for x:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        panic!("Input was empty. Please enter a valid number.");
    }

    let y: i32 = trimmed_input.parse::<i32>().expect("Please enter a valid number") + 2;

    println!("The value of y is: {}", y);
}
