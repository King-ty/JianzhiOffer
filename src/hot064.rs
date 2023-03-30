struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for i in 0..m {
            for j in 0..n {
                let mut temp = i32::MAX;
                if i > 0 {
                    temp = temp.min(grid[i - 1][j]);
                }
                if j > 0 {
                    temp = temp.min(grid[i][j - 1]);
                }
                if temp < i32::MAX {
                    grid[i][j] += temp;
                }
            }
        }
        grid[m - 1][n - 1]
    }
}
