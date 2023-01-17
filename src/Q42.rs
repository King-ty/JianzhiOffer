struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut ret = nums[0];
        let mut nw = 0;
        for num in nums {
            nw += num;
            ret = max(ret, nw);
            if nw < 0 {
                nw = 0;
            }
        }
        ret
    }
}
