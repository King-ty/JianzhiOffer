struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        const MV: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
        let m = board.len() as i32;
        if m == 0 {
            return false;
        }
        let n = board[0].len() as i32;
        if n == 0 {
            return false;
        }
        let mut vs = vec![vec![false; n as usize]; m as usize];
        let chars: Vec<char> = word.chars().collect();
        fn search(
            board: &Vec<Vec<char>>,
            chars: &Vec<char>,
            pos: (i32, i32),
            scope: (i32, i32),
            ind: usize,
            vs: &mut Vec<Vec<bool>>,
        ) -> bool {
            if chars[ind] == board[pos.0 as usize][pos.1 as usize] {
                if ind == chars.len() - 1 {
                    return true;
                }
                vs[pos.0 as usize][pos.1 as usize] = true;
                for mv in MV {
                    if pos.0 + mv[0] < scope.0
                        && pos.0 + mv[0] >= 0
                        && pos.1 + mv[1] < scope.1
                        && pos.1 + mv[1] >= 0
                        && !vs[(pos.0 + mv[0]) as usize][(pos.1 + mv[1]) as usize]
                    {
                        if search(
                            board,
                            chars,
                            (pos.0 + mv[0], pos.1 + mv[1]),
                            scope,
                            ind + 1,
                            vs,
                        ) {
                            // vs[pos.0 as usize][pos.1 as usize] = false;
                            return true;
                        }
                    }
                }
            }
            vs[pos.0 as usize][pos.1 as usize] = false;
            false
        }
        for i in 0..m {
            for j in 0..n {
                if board[i as usize][j as usize] == chars[0]
                    && search(&board, &chars, (i, j), (m, n), 0, &mut vs)
                {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q12() {
        use super::Solution;
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        // let word = "ABCB".to_string();
        // let word = "SEE".to_string();
        let word = "ABCCED".to_string();
        assert_eq!(true, Solution::exist(board, word));
    }
}
