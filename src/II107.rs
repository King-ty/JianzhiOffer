struct Solution;

// 动态规划，一开始觉得像想了一下，没有想到对角线走2遍的方法，甚至没想到走4遍的解法...
// res其实也可以优化掉，降低空间
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp;
        let (m, n) = (mat.len(), mat[0].len());
        let mut res = vec![vec![i32::MAX / 2; n]; m];
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    res[i][j] = 0;
                } else {
                    if i > 0 {
                        res[i][j] = cmp::min(res[i][j], res[i - 1][j] + 1);
                    }
                    if j > 0 {
                        res[i][j] = cmp::min(res[i][j], res[i][j - 1] + 1);
                    }
                }
            }
        }
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i < m - 1 {
                    res[i][j] = cmp::min(res[i][j], res[i + 1][j] + 1);
                }
                if j < n - 1 {
                    res[i][j] = cmp::min(res[i][j], res[i][j + 1] + 1);
                }
            }
        }
        res
    }
}
