// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let sum = add(34.0, 67.0);
    show(sum);
}

fn add(a: f64, b: f64) -> f64 {
    a+b
}

fn show(s: f64){
    println!("{:?}", s);
}