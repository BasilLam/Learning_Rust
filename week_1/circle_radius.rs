use std::io;
const PI: f64 = 3.14;
fn main()
{
    println!("Please enter the radius of the circle");

    let mut radius_input = String::new();
    io::stdin().read_line(&mut radius_input).expect("Failed to read line");
    if radius_input.is_empty() {
        panic!("Input was empty - Please provide a valid number")
    }
    let radius: f64 = radius_input.trim().parse().expect("Please enter a valid number");

    if radius <= 0.0 {
        panic!("Radius is negative or zero - Please provide a valid number")
    }
    let radius_sq : f64 = radius.powi(2);

    let a: f64 = radius_sq * PI;

    println!("The area of the circle is: {}", a);
}