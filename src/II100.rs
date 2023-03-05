struct Solution;

// 也可以从下到上
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        use std::cmp;
        let m = triangle.len();
        let mut dp = vec![i32::MAX; m + 1];
        dp[1] = 0;
        for i in 0..m {
            for j in (0..triangle[i].len()).rev() {
                dp[j + 1] = cmp::min(dp[j], dp[j + 1]) + triangle[i][j];
            }
        }
        let mut res = i32::MAX;
        for val in dp {
            res = cmp::min(res, val);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test_II100() {
        assert_eq!(
            11,
            dbg!(Solution::minimum_total(vec![
                vec![2],
                vec![3, 4],
                vec![6, 5, 7],
                vec![4, 1, 8, 3]
            ]))
        );
    }
}
