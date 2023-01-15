pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut l = 0 as i32;
        let mut r = nums.len() as i32;
        let mut mid;
        while l < r {
            mid = l + (r - l) / 2;
            if nums[mid as usize] == mid {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }
}
