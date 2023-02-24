struct Solution;

// 偶数二分法
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mut mid = l + (r - l) / 2;
            mid -= mid & 1;
            if nums[mid] == nums[mid + 1] {
                l = mid + 2;
            } else {
                r = mid;
            }
        }
        return nums[l];
    }
}
