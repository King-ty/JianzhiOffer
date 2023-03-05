struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp;
        fn search_area(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
            const MOVS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
            let (m, n) = (grid.len(), grid[0].len());
            let mut res = 0;
            let mut st = vec![(x, y)];
            grid[x][y] = 0;
            while let Some((x, y)) = st.pop() {
                res += 1;
                for mv in MOVS {
                    if x as i32 + mv[0] >= 0
                        && x as i32 + mv[0] < m as i32
                        && y as i32 + mv[1] >= 0
                        && y as i32 + mv[1] < n as i32
                    {
                        let (xx, yy) = ((x as i32 + mv[0]) as usize, (y as i32 + mv[1]) as usize);
                        if grid[xx][yy] > 0 {
                            st.push((xx, yy));
                            grid[xx][yy] = 0;
                        }
                    }
                }
            }
            dbg!(res)
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 0 {
                    res = cmp::max(res, search_area(&mut grid, i, j));
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II105() {
        assert_eq!(
            4,
            dbg!(Solution::max_area_of_island(vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1],
                vec![0, 0, 0, 1, 1]
            ]))
        );
    }
}
