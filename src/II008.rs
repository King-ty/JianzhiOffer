struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        use std::cmp::min;
        let mut sm = 0;
        let mut l = 0;
        let mut ret = 0;
        for i in 0..nums.len() {
            sm += nums[i];
            if sm < target {
                continue;
            }
            if ret == 0 {
                ret = (i - l + 1) as i32;
            }
            while sm >= target {
                sm -= nums[l];
                l += 1;
            }
            ret = min(ret, (i - l + 2) as i32);
        }
        ret
    }
}
