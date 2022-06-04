use std::{
    // io, 
    // io::Write, 
    // thread, 
    // time,
    convert::TryInto,
    convert::TryFrom,
};


struct  Solution {}
impl    Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (len_1, len_2) : (usize, usize) = (nums1.len(), nums2.len());
        let len_3 = len_1+len_2;
        let (mut i_1, mut i_2) : (usize, usize) = (0, 0);
        let (mut median_1, mut median_2) : (i32, i32) = (0,0);
        for i in 0..=(len_3/2) {
            if {
                if (i_1 < len_1) && (i_2 < len_2)
                {nums1[i_1] < nums2[i_2]}
                else
                {i_2 < len_2}
            }
            { // chooses list1 for median
                i_1 += 1;
                median_1 = median_2;
                median_2 = nums1[i_1];
            }
            else 
            {
                i_2 += 1;
                median_1 = median_2;
                median_2 = nums2[i_2];
            }
        }
        return {
            if (len_3 & 1) == 1 // if odd
            {median_2.into()}
            else
            {f64::from(median_1+median_2)/2_f64}
        };
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1(){
        let nums1 = vec![1,3];
        let nums2 = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(nums1,nums2),2_f64);
    }
}