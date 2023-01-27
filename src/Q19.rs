struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut f = vec![vec![false; s.len() + 1]; p.len() + 1];
        let s = s.into_bytes();
        let p = p.into_bytes();
        f[0][0] = true;
        for i in 0..p.len() {
            f[i + 1][0] = p[i] == '*' as u8 && f[i - 1][0];
            for j in 0..s.len() {
                if p[i] != '.' as u8 && p[i] != '*' as u8 {
                    f[i + 1][j + 1] = f[i][j] && p[i] == s[j];
                } else if p[i] == '.' as u8 {
                    f[i + 1][j + 1] = f[i][j];
                } else {
                    f[i + 1][j + 1] = f[i - 1][j + 1]
                        || (f[i + 1][j] && (p[i - 1] == '.' as u8 || s[j] == p[i - 1]));
                }
            }
        }
        f[p.len()][s.len()]
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q19() {
        use super::Solution;
        assert_eq!(
            true,
            dbg!(Solution::is_match("aab".to_string(), "..*".to_string()))
        );
    }
}
