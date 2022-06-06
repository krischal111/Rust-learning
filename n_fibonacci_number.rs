use std::{io, io::Write};
fn main() {
    print!("Enter the value of n = ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i32 = input.trim().parse::<i32>().unwrap();
    println!("The {n}th fibonacci number is {}", nth_fibonacci_number(n));
}

fn nth_fibonacci_number(n: i32) -> i32 {
    if n < 2 {
        1
    } else {
        nth_fibonacci_number(n - 1) + nth_fibonacci_number(n - 2)
    }
}
