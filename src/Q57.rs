struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            match (nums[l] + nums[r]).cmp(&target) {
                Ordering::Equal => return vec![nums[l], nums[r]],
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
            }
        }
        Vec::new()
    }
}
