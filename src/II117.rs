struct Solution;

// 并查集，不知道为什么提交上去超过很少
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        fn is_connected(s1: &str, s2: &str) -> bool {
            let mut cnt = 0;
            for (c1, c2) in s1.bytes().zip(s2.bytes()) {
                if c1 != c2 {
                    cnt += 1;
                }
            }
            return cnt == 2 || cnt == 0;
        }
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
        let n = strs.len();
        let mut fa: Vec<usize> = (0..n).collect();
        for i in 1..n {
            for j in 0..i {
                if is_connected(&strs[i], &strs[j]) {
                    merge(&mut fa, i, j);
                }
            }
        }
        (0..n).filter(|&x| fa[x] == x).count() as i32
    }
}
