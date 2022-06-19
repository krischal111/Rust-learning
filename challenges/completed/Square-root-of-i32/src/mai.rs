struct Solution {}
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        (sqrt(x as f64)) as i32
    }
}

fn main() {
    let n = 2147302921;
    print!("The square root of {n} is ");
    io::stdout().flush().unwrap();
    let sqrt = Solution::my_sqrt(n);
    println!("\n{sqrt} ");
}
