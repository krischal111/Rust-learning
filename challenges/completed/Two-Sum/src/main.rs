struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i1, mut i2): (usize, usize) = (0, 0);
        'outer: for (i,value) in nums.iter().enumerate() {
        //i in 0..(nums.len() - 1) {
            let to_find = target - value;

            // Logic 1
            // let position = nums.iter().position(|x| *x == to_find);
            // match position {
            //     None => (),
            //     Some(x) => {
            //         i1 = i;
            //         i2 = x;
            //         if (nums[x] == to_find) {
            //             break 'outer;
            //         }
            //     }
            // }

            // Logic 0
            for j in (i+1)..nums.len()
            {
                if to_find == nums[j]
                {
                    i1 = i;
                    i2 = j;
                    break 'outer;
                }
            }
        }
        return vec![i1 as i32, i2 as i32];
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
        let nums = vec![2, 7, 11, 15];
        let target: i32 = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
    #[test]
    fn test2() {
        let nums = vec![3, 2, 4];
        let target: i32 = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);
    }
    #[test]
    fn test3() {
        let nums = vec![3, 3];
        let target: i32 = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
