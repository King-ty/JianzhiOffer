struct Solution;

// 01背包
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let goal: i32 = nums.iter().sum();
        if goal & 1 == 1 {
            return false;
        }
        let goal = (goal / 2) as usize;
        let mut dp = vec![false; goal + 1];
        dp[0] = true;
        for num in nums {
            for j in (num as usize..=goal).rev() {
                dp[j] = dp[j] || dp[j - num as usize]
            }
        }
        dp[goal]
    }
}
