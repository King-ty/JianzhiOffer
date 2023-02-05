struct Solution;

/// Manacher马拉车，很巧妙的算法！
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        use std::cmp::min;
        let mut ss = "$#".to_string();
        for c in s.chars() {
            ss.push(c);
            ss.push('#');
        }
        ss.push('!');
        let ss = ss.as_bytes();
        let n = ss.len();
        let (mut rm, mut im) = (0, 0);
        let mut f = vec![0; n];
        let mut ret = 0;
        for i in 1..n - 1 {
            f[i] = if rm >= i {
                min(rm - i + 1, f[im * 2 - i])
            } else {
                1
            };
            while ss[i + f[i]] == ss[i - f[i]] {
                f[i] += 1;
            }
            if i + f[i] - 1 > rm {
                rm = i + f[i] - 1;
                im = i;
            }
            ret += f[i] / 2;
        }
        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II020() {
        assert_eq!(6, dbg!(Solution::count_substrings("aaa".to_string())));
    }
}
