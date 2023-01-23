struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len() as i32;
        if m == 0 {
            return Vec::new();
        }
        let n = matrix[0].len() as i32;
        let mut vs = Vec::new();
        for _ in 0..m {
            let mut temp = Vec::new();
            temp.resize(n as usize, false);
            vs.push(temp);
        }
        const MV: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let mut state = 0;
        let (mut x, mut y) = (0i32, 0i32);
        let mut ret = Vec::new();
        for _ in 0..m * n {
            ret.push(matrix[x as usize][y as usize]);
            vs[x as usize][y as usize] = true;
            if x + MV[state][0] >= m
                || x + MV[state][0] < 0
                || y + MV[state][1] >= n
                || y + MV[state][1] < 0
                || vs[(x + MV[state][0]) as usize][(y + MV[state][1]) as usize]
            {
                state = (state + 1) % 4;
            }
            x += MV[state][0];
            y += MV[state][1];
        }
        ret
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q29() {
        use super::Solution;
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            dbg!(Solution::spiral_order(vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9]
            ]))
        );
    }
}
