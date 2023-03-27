struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut longest = 0;
        for (i, num) in nums.into_iter().enumerate() {
            if i > longest {
                return false;
            }
            longest = longest.max(i + num as usize);
        }
        true
    }
}
