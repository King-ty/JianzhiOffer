struct Solution;

// 全二分，但是用位运算简化奇偶判断
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mid = l + (r - l) / 2;

            // 这里^1运算导致奇数和上一个比，偶数和下一个比，自动分辨奇偶
            if nums[mid] == nums[mid ^ 1] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        return nums[l];
    }
}
