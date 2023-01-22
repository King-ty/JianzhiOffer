// 摩尔投票法
struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut ret = nums[0];
        let mut votes = 0;
        for num in nums {
            if votes == 0 {
                ret = num;
            }
            if num == ret {
                votes += 1;
            } else {
                votes -= 1;
            }
        }
        ret
    }
}
