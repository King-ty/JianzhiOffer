struct Solution;

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        let mut prev = [0, 0, 0];
        for cost in costs {
            prev = [
                min(prev[1], prev[2]) + cost[0],
                min(prev[0], prev[2]) + cost[1],
                min(prev[0], prev[1]) + cost[2],
            ];
        }
        min(prev[0], min(prev[1], prev[2]))
    }
}
