struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        if nums[0] == target {
            return 0;
        } else if nums[n - 1] == target {
            return n as i32 - 1;
        }
        let left_part = nums[0] < target;
        let (mut l, mut r) = (0, n);
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[0] < nums[mid] {
                // 中间点在左侧
                if nums[mid] < target || !left_part {
                    // 中间点在target左侧 或 target在右侧
                    l = mid + 1;
                } else {
                    // target在左侧且在中间点左侧
                    r = mid;
                }
            } else {
                // 中间点在右侧
                if nums[mid] > target || left_part {
                    // 中间点在target右侧 或 target在左侧
                    r = mid;
                } else {
                    // target在右侧且在中间点右侧
                    l = mid + 1;
                }
            }
        }
        -1
    }
}
