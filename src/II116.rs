struct Solution;

// 并查集
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        fn find_root(fa: &mut Vec<usize>, x: usize) -> usize {
            if fa[x] == x {
                x
            } else {
                fa[x] = find_root(fa, fa[x]);
                fa[x]
            }
        }
        fn merge(fa: &mut Vec<usize>, x: usize, y: usize) {
            let (rootx, rooty) = (find_root(fa, x), find_root(fa, y));
            fa[rootx] = rooty;
        }
        let n = is_connected.len();
        let mut fa: Vec<usize> = (0..n).collect();
        for i in 0..n {
            for j in 0..n {
                if is_connected[i][j] == 1 {
                    merge(&mut fa, i, j);
                }
            }
        }
        (0..n).filter(|&x| fa[x] == x).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II116() {
        assert_eq!(
            2,
            dbg!(Solution::find_circle_num(vec![
                vec![1, 1, 0],
                vec![1, 1, 0],
                vec![0, 0, 1]
            ]))
        )
    }
}
