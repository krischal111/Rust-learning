use std::convert::TryInto;
use std::{io, thread, time};
use std::io::Write;

fn delay()
{
    let delay_time = time::Duration::from_millis(100);
    thread::sleep(delay_time);
}
enum Comparison {Less, Equal, Greater}
impl Comparison
{
    pub fn square_check (a:i64, b:i64) -> Comparison // less means square of `a` is less than `b`
    {
        let sq1 = a*a;
        let sq2 = sq1 + 1 + 2 * a;

        if (sq1 <= b) && (b < sq2) {
            Comparison::Equal
        } else if b < sq2
        {Comparison::Greater}
        else
        {Comparison::Less}
    }
}
struct Solution {}
impl   Solution {
    pub fn my_sqrt(x:i32) -> i32
    {
        let mut a:i64 = 0i64; 
        let mut b:i64 = x.into();
        let xx:i64 = x.into();
        let mut avg:i64 = x.into();
        return loop
        {
            match Comparison::square_check (avg, xx)
            {
                Comparison::Less => a = avg,
                Comparison::Greater => b = avg,
                Comparison::Equal   => break avg.try_into().unwrap(),
            }
            // print!("\nBetween [{a}, {b}] checking {avg} * {avg} = {} for root of {x}", avg * avg);
            // io::stdout().flush().unwrap();
            // delay();
            avg = (a+b)/2;
        }
    }
}

fn main() {
    let n = 2147395599;
    print!("The square root of {n} is ");
    io::stdout().flush().unwrap();
    let sqrt = Solution::my_sqrt(n);
    println!("\n{sqrt} ");
}