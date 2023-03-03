use std::vec;

struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        use std::cmp::max;
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
        let (m, n) = (text1.len(), text2.len());
        let mut f = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                if text1[i] == text2[j] {
                    f[i + 1][j + 1] = f[i][j] + 1;
                } else {
                    f[i + 1][j + 1] = max(f[i + 1][j], f[i][j + 1]);
                }
            }
        }
        f[m][n]
    }
}
