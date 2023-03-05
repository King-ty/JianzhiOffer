struct Solution;

// sum-2*neg=target => neg=(sum-target)/2
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sm: i32 = nums.iter().sum();
        if sm < target || (sm - target) & 1 == 1 {
            return 0;
        }
        let neg = ((sm - target) / 2) as usize;
        let mut dp = vec![0; neg + 1];
        dp[0] = 1;
        for num in nums {
            let num = num as usize;
            for j in (num..=neg).rev() {
                dp[j] += dp[j - num];
            }
        }
        dp[neg]
    }
}
