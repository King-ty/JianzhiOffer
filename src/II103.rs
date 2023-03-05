use std::vec;

struct Solution;

// 多重背包
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        use std::cmp;
        let amount = amount as usize;
        let mut dp = vec![amount + 1; amount + 1];
        dp[0] = 0;
        for coin in coins {
            for j in coin as usize..=amount {
                dp[j] = cmp::min(dp[j], dp[j - coin as usize] + 1);
            }
        }
        if dp[amount] == amount + 1 {
            -1
        } else {
            dp[amount] as i32
        }
    }
}
