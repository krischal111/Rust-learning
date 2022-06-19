struct Solution {}
impl Solution {
    pub fn my_atoi(input: String) -> i32 {
        let mut a: i64 = 0;
        let mut leading = true;
        let mut negative = false;
        for i in input.chars() {
            leading &= (i == ' ') || (i == '+') || (i == '-');
            if leading {
                if i == '-' {
                    negative = !negative;
                    leading = false;
                }
                if i == '+' {
                    leading = false;
                }
                continue;
            }
            if i.is_numeric() {
                a *= 10;
                a += i as i64;
                a -= '0' as i64;
                a = if negative {
                    if a > -(i32::MIN as i64) {
                        -(i32::MIN as i64)
                    } else {
                        a
                    }
                } else {
                    if a > (i32::MAX as i64) {
                        i32::MAX as i64
                    } else {
                        a
                    }
                }
            } else {
                break;
            }
            // println!("Next iteration a = {a}");
        }
        if negative {
            a = -a;
        }
        // println!("Final iteration a = {a}");
        // println!("After casting a as i32 = {}", a as i32);
        return a as i32;
    }
}

fn main() {
    println!("{}", Solution::my_atoi(String::from("-91283472332")));
}
