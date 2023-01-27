// 看错题意了，这里相当于认为*匹配任意字符串的思路，也挺有意思的，先保留了
struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut f = vec![vec![false; s.len() + 1]; p.len() + 1];
        let mut g = vec![false; p.len() + 1];
        let s = s.into_bytes();
        let p = p.into_bytes();
        f[0][0] = true;
        g[0] = true;
        for i in 0..p.len() {
            for j in 0..s.len() {
                if p[i] != '.' as u8 && p[i] != '*' as u8 {
                    f[i + 1][j + 1] = f[i][j] && p[i] == s[j];
                } else if p[i] == '.' as u8 {
                    f[i + 1][j + 1] = f[i][j];
                } else {
                    f[i + 1][j + 1] = g[i];
                }
                g[i + 1] = g[i + 1] || f[i + 1][j + 1];
            }
        }
        f[p.len()][s.len()]
    }
}
