pub struct Solution;

impl Solution {
    fn lowerbound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut l = 0 as i32;
        let mut r = nums.len() as i32;
        let mut mid;
        while l < r {
            mid = l + (r - l) / 2;
            if nums[mid as usize] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Solution::lowerbound(&nums, target + 1) - Solution::lowerbound(&nums, target)
    }
}
