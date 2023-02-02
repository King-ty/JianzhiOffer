struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let mut l = 0;
        let mut sm = 1;
        for i in 0..nums.len() {
            sm *= nums[i];
            while sm >= k && l <= i {
                sm /= nums[l];
                l += 1;
            }
            ret += i - l + 1;
        }
        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test_II009() {
        assert_eq!(
            1,
            dbg!(Solution::num_subarray_product_less_than_k(vec![5, 2, 3], 0))
        );
    }
}
