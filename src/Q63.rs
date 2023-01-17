struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        use std::cmp::{max, min};
        let mut lowest = prices[0];
        let mut ret = 0;
        for i in 1..prices.len() {
            ret = max(ret, prices[i] - lowest);
            lowest = min(lowest, prices[i]);
        }
        ret
    }
}
