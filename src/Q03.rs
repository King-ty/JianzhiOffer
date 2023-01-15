use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut hs = HashSet::new();
        for num in nums {
            if hs.contains(&num) {
                return num;
            }
            hs.insert(num);
        }
        0
    }
}
