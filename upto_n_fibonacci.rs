use std::{io, io::Write};
fn main() {
    print!("Enter the value of n = ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i32 = input.trim().parse::<i32>().unwrap();
    println!(
        "The fibonacci numbers upto {n} terms are {:?}",
        upto_n_fibonacci_number(n)
    );
}

fn upto_n_fibonacci_number(n: i32) -> Vec<i32> {
    let mut fib_vec = vec![];
    let (mut a, mut b, mut c): (i32, i32, i32) = (0, 0, 1);
    for _i in 0..n {
        fib_vec.push(c);
        a = b;
        b = c;
        c = a + b;
    }
    return fib_vec;
}
