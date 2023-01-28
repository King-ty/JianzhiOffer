struct Solution;

impl Solution {
    pub fn dices_probability(n: i32) -> Vec<f64> {
        let mut f = vec![0.0; (5 * n + 1) as usize];
        let mut temp = vec![0.0; (5 * n + 1) as usize];
        f[0] = (1.0 as f64 / 6.0).powi(n);
        temp[0] = f[0];
        for _ in 0..n {
            for j in 1..=n * 5 {
                let j = j as usize;
                temp[j] = f[j];
                f[j] += f[j - 1];
                if j >= 6 {
                    f[j] -= temp[j - 6];
                }
            }
        }
        f
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q60_2() {
        use super::Solution;
        assert_eq!(
            vec![
                0.02778, 0.05556, 0.08333, 0.11111, 0.13889, 0.16667, 0.13889, 0.11111, 0.08333,
                0.05556, 0.02778
            ],
            dbg!(Solution::dices_probability(2))
        );
    }
}
