use std::convert::TryFrom;
struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i1, mut i2) : (usize, usize)  = (0, 0);
        'outer: for i in 0..(nums.len()-1)
        {
            for j in (i+1)..nums.len()
            {
                if target == (nums[i] + nums[j])
                {
                    i1 = i;
                    i2 = j;
                    break 'outer;
                }
            }
        }
        return vec![i32::try_from(i1).unwrap(), i32::try_from(i2).unwrap()];
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let nums = vec![2,7,11,15];
        let target:i32 = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0,1]);
    }
    #[test]
    fn test2() {
        let nums = vec![3,2,4];
        let target:i32 = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![1,2]);
    }
    #[test]
    fn test3() {
        let nums = vec![3,3];
        let target:i32 = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![0,1]);
    }
    
}