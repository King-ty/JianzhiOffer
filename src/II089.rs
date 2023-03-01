struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut prev = [0; 2];
        for num in nums {
            let temp = [max(prev[0], prev[1]), prev[0] + num];
            prev = temp;
        }
        max(prev[0], prev[1])
    }
}
