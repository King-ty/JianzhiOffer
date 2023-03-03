struct Solution;

// 马拉车+动态规划，复杂度比题解略好
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        use std::cmp;
        let ss = "$#".to_string()
            + &(s
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("#"))
            + "#!";
        let ss = ss.as_bytes();
        let nn = ss.len();
        let mut f = vec![0; nn];
        let (mut rm, mut im) = (0, 0);
        for i in 1..nn - 1 {
            f[i] = if i <= rm {
                cmp::min(rm - i + 1, f[im * 2 - i])
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
        }
        let n = s.len();
        let mut dp = vec![0; n];
        for i in 1..n {
            dp[i] = i;
            for j in 0..=i {
                // 注意j枚举到=i
                let ind = i + j + 2; // 注意这里的+2是推算出来的
                if f[ind] - 1 >= i - j + 1 {
                    // f[ind]-1为回文串长度
                    dp[i] = if j > 0 {
                        cmp::min(dp[i], 1 + dp[j - 1])
                    } else {
                        0 // 注意这种情况是0
                    };
                }
            }
        }
        dp[n - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II094() {
        assert_eq!(1, dbg!(Solution::min_cut("aab".to_string())));
        assert_eq!(1, dbg!(Solution::min_cut("ab".to_string())));
        assert_eq!(0, dbg!(Solution::min_cut("bb".to_string())));
        assert_eq!(0, dbg!(Solution::min_cut("b".to_string())));
    }
}
