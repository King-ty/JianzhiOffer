struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        fn match_u8(x: u8, y: u8) -> bool {
            x == b'.' || x == y
        }
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (m, n) = (s.len(), p.len());
        let mut f = vec![vec![false; m + 1]; n + 1];
        f[0][0] = true;
        for i in 0..n {
            if p[i] == b'*' {
                f[i + 1][0] = f[i - 1][0];
            }
            for j in 0..m {
                if p[i] == b'.' {
                    f[i + 1][j + 1] = f[i][j];
                } else if p[i] == b'*' {
                    f[i + 1][j + 1] =
                        f[i - 1][j + 1] || f[i][j + 1] || (f[i + 1][j] && match_u8(p[i - 1], s[j]));
                } else {
                    f[i + 1][j + 1] = f[i][j] && p[i] == s[j];
                }
            }
        }
        f[n][m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II006() {
        assert_eq!(
            false,
            (Solution::is_match("aa".to_string(), "a".to_string()))
        );
        assert_eq!(
            true,
            (Solution::is_match("aa".to_string(), "a*".to_string()))
        );
        assert_eq!(
            true,
            (Solution::is_match("ab".to_string(), ".*".to_string()))
        );
        assert_eq!(
            true,
            (Solution::is_match("aab".to_string(), "c*a*b".to_string()))
        );
        assert_eq!(
            true,
            (Solution::is_match("a".to_string(), "a*c*b*".to_string()))
        );
    }
}
