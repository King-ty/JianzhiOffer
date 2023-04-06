struct Solution;

// 朴素深搜
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        use std::collections::HashSet;
        fn dfs(
            board: &Vec<Vec<char>>,
            bytes: &[u8],
            vis: &mut HashSet<(usize, usize)>,
            x: usize,
            y: usize,
            ind: usize,
        ) -> bool {
            if ind == bytes.len() {
                return true;
            }
            vis.insert((x, y));
            const MOV: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
            let (m, n) = (board.len(), board[0].len());
            for mov in MOV {
                let (xx, yy) = (x as i32, y as i32);
                if xx + mov[0] < 0
                    || xx + mov[0] >= m as i32
                    || yy + mov[1] < 0
                    || yy + mov[1] >= n as i32
                {
                    continue;
                }
                let (xx, yy) = ((xx + mov[0]) as usize, (yy + mov[1]) as usize);
                if !vis.contains(&(xx, yy))
                    && board[xx][yy] == bytes[ind] as char
                    && dfs(board, bytes, vis, xx, yy, ind + 1)
                {
                    return true;
                }
            }
            vis.remove(&(x, y));
            false
            // if x > 0
            //     && !vis.contains(&(x - 1, y))
            //     && board[x - 1][y] == bytes[ind] as char
            //     && dfs(board, bytes, vis, x - 1, y, ind + 1)
            // {
            //     true
            // } else if x < board.len() - 1
            //     && !vis.contains(&(x + 1, y))
            //     && board[x + 1][y] == bytes[ind] as char
            //     && dfs(board, bytes, vis, x + 1, y, ind + 1)
            // {
            //     true
            // } else if y > 0
            //     && !vis.contains(&(x, y - 1))
            //     && board[x][y - 1] == bytes[ind] as char
            //     && dfs(board, bytes, vis, x, y - 1, ind + 1)
            // {
            //     true
            // } else if y < board[0].len() - 1
            //     && !vis.contains(&(x, y + 1))
            //     && board[x][y + 1] == bytes[ind] as char
            //     && dfs(board, bytes, vis, x, y + 1, ind + 1)
            // {
            //     true
            // } else {
            //     false
            // }
        }

        let (m, n) = (board.len(), board[0].len());
        let bytes = word.as_bytes();
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == bytes[0] as char
                    && dfs(&board, bytes, &mut HashSet::new(), i, j, 1)
                {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hot079() {
        assert_eq!(
            false,
            dbg!(Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_string()
            ))
        );
    }
}
