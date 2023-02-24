struct Solution;

// 朴素分奇偶判断法
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mid = l + (r - l) / 2;
            if mid & 1 == 0 {
                // mid 偶数
                if nums[mid] == nums[mid + 1] {
                    l = mid + 2;
                } else if mid == 0 || nums[mid] != nums[mid - 1] {
                    return nums[mid];
                } else {
                    r = mid - 2;
                }
            } else {
                // mid 奇数
                if nums[mid] == nums[mid - 1] {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }
        return nums[l];
    }
}
