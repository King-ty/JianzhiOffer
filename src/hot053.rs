use core::num;

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp;
        let mut cur = 0;
        let mut ret = nums[0];
        for num in nums {
            cur = cmp::max(num, cur + num);
            ret = ret.max(cur);
        }
        ret
    }
}
