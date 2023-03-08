struct Solution;

// 记忆化搜索
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        use std::cmp;
        fn dfs(
            matrix: &Vec<Vec<i32>>,
            lens: &mut Vec<Vec<i32>>,
            x: usize,
            y: usize,
            m: usize,
            n: usize,
        ) {
            const MOVS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
            for mv in MOVS {
                let (xx, yy) = (x as i32 + mv[0], y as i32 + mv[1]);
                if xx >= 0 && xx < m as i32 && yy >= 0 && yy < n as i32 {
                    let (xx, yy) = (xx as usize, yy as usize);
                    if matrix[xx][yy] > matrix[x][y] {
                        if lens[xx][yy] == 0 {
                            dfs(matrix, lens, xx, yy, m, n);
                        }
                        lens[x][y] = cmp::max(lens[x][y], lens[xx][yy]);
                    }
                }
            }
            // 防止最高点重复浪费时间，是很关键的优化
            lens[x][y] += 1;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut lens = vec![vec![0; n]; m];
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if lens[i][j] == 0 {
                    dfs(&matrix, &mut lens, i, j, m, n);
                    res = cmp::max(res, lens[i][j]);
                }
            }
        }
        res
    }
}
