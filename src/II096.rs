struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (m, n, mn) = (s1.len(), s2.len(), s3.len());
        if m + n != mn {
            return false;
        }
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;

        for j in 0..n {
            dp[0][j + 1] = dp[0][j] && s2[j] == s3[j];
        }
        for i in 0..m {
            dp[i + 1][0] = dp[i][0] && s1[i] == s3[i];
            for j in 0..n {
                dp[i + 1][j + 1] = (dp[i][j + 1] && s1[i] == s3[i + j + 1])
                    || (dp[i + 1][j] && s2[j] == s3[i + j + 1]);
            }
        }
        dp[m][n]
    }
}
