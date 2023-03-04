struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (m, n) = (s.len(), t.len());
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 0..m {
            for j in (0..n).rev() {
                if s[i] == t[j] {
                    dp[j + 1] += dp[j];
                }
            }
        }
        dp[n]
    }
}

// impl Solution {
//     pub fn num_distinct(s: String, t: String) -> i32 {
//         let (m, n) = (s.len(), t.len());
//         let (s, t) = (s.as_bytes(), t.as_bytes());
//         let mut dp = vec![vec![0; n + 1]; m + 1];
//         dp[0][0] = 1;
//         for i in 0..m {
//             for j in 0..n {
//                 dp[i + 1][j + 1] = dp[i][j + 1];
//                 if s[i] == t[j] {
//                     dp[i + 1][j + 1] += dp[i][j];
//                 }
//             }
//         }
//         dp[m][n]
//     }
// }
