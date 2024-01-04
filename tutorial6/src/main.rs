// overflowing and typecasting
use std::io;

fn main() {
    let test = i32::MAX;
    let x = 255f64;
    let y = 2 as i32;
    let z = x / (y as f64);

    println!("Z: {}", z);
    println!("test {}", test);


    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error reading line");

    println!("Input{}", input);

    let int_input: u64  = input.trim().parse().unwrap();
    println!("int input {}", int_input);

}
