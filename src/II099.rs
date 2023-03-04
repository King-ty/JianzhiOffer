struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp;
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        for i in 0..m {
            for j in 0..n {
                dp[j + 1] = cmp::min(dp[j + 1], dp[j]) + grid[i][j];
            }
            dp[0] = i32::MAX;
        }
        dp[n]
    }
}

// impl Solution {
//     pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
//         use std::cmp;
//         let (m, n) = (grid.len(), grid[0].len());
//         let mut dp = vec![vec![0; n + 1]; m + 1];
//         for i in 0..m {
//             for j in 0..n {
//                 dp[i + 1][j + 1] = cmp::min(dp[i][j + 1], dp[i + 1][j]) + grid[i][j];
//             }
//         }
//         dp[m][n]
//     }
// }
