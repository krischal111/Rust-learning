use std::convert::TryFrom;

struct Solution {}
impl Solution {
    pub fn find_median_sorted_arrays(nums_1: Vec<i32>, nums_2: Vec<i32>) -> f64 {
        let (len_1, len_2): (usize, usize) = (nums_1.len(), nums_2.len());
        let len_3 = len_1 + len_2;
        let (mut i_1, mut i_2): (usize, usize) = (0, 0);
        let (mut median_1, mut median_2): (i32, i32) = (0, 0);
        for _i in 0..=(len_3 / 2) {
            if {
                if (i_1 < len_1) && (i_2 < len_2) {
                    nums_1[i_1] < nums_2[i_2]
                } else {
                    i_1 < len_1
                }
            } {
                // chooses nums_1 for median
                median_1 = median_2;
                median_2 = nums_1[i_1];
                i_1 += 1;
            } else {
                median_1 = median_2;
                median_2 = nums_2[i_2];
                i_2 += 1;
            }
        }
        return {
            if (len_3 & 1) == 1
            // if odd
            {
                median_2.into()
            } else {
                f64::from(median_1 + median_2) / 2_f64
            }
        };
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let nums_1 = vec![1, 3];
        let nums_2 = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(nums_1, nums_2), 2_f64);
    }
}
