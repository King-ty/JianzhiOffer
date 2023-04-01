struct Solution;

impl Solution {
    pub fn min_distance(mut word1: String, mut word2: String) -> i32 {
        // 保证最后是相等的
        word1.push('*');
        word2.push('*');
        let (len1, len2) = (word1.len(), word2.len());
        let mut f = vec![vec![i32::MAX / 2; len1 + 1]; len2 + 1];
        f[0][0] = 0;
        for i in 0..len2 {
            for j in 0..len1 {
                if word2.as_bytes()[i] == word1.as_bytes()[j] {
                    // 匹配
                    f[i + 1][j + 1] = f[i + 1][j + 1].min(f[i][j]);
                } else {
                    // 减少
                    f[i][j + 1] = f[i][j + 1].min(f[i][j] + 1);
                    // 增加
                    f[i + 1][j] = f[i + 1][j].min(f[i][j] + 1);
                    // 改变
                    f[i + 1][j + 1] = f[i + 1][j + 1].min(f[i][j] + 1);
                }
            }
        }
        f[len2][len1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hot072() {
        assert_eq!(
            3,
            dbg!(Solution::min_distance(
                "horse".to_string(),
                "ros".to_string()
            ))
        );
    }
}
