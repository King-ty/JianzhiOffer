struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![1; n + 1];
        for _ in 1..m {
            for j in 1..n {
                dp[j + 1] += dp[j];
            }
        }
        dp[n]
    }
}

// impl Solution {
//     pub fn unique_paths(m: i32, n: i32) -> i32 {
//         let (m, n) = (m as usize, n as usize);
//         let mut dp = vec![vec![1; n + 1]; m + 1];
//         for i in 1..m {
//             for j in 1..n {
//                 dp[i + 1][j + 1] = dp[i][j + 1] + dp[i + 1][j];
//             }
//         }
//         dp[m][n]
//     }
// }
