struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        use std::cmp::min;
        cost.push(0);
        let mut f0;
        let (mut f1, mut f2) = (cost[0], cost[1]);
        for i in 2..cost.len() {
            f0 = f1;
            f1 = f2;
            f2 = min(f0, f1) + cost[i];
        }
        f2
    }
}
