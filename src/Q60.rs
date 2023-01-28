// 讲真这题我一开始思路只有枚举，但这显然不好
struct Solution;

impl Solution {
    pub fn dices_probability(n: i32) -> Vec<f64> {
        fn dfs(cur: i32, n: i32, ss: i32, delt: f64, ret: &mut Vec<f64>) {
            if cur == 0 {
                ret[(ss - n) as usize] += delt;
                return;
            }
            for i in 1..=6 {
                dfs(cur - 1, n, ss + i, delt, ret);
            }
        }
        let delt = (1.0 as f64 / 6.0).powi(n);
        let mut ret = vec![0.0; (5 * n + 1) as usize];
        dfs(n, n, 0, delt, &mut ret);
        ret
    }
}
