struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![0; n];
        f[0] = 1;
        for _ in 0..m {
            for j in 1..n {
                f[j] += f[j - 1];
            }
        }
        f[n - 1]
    }
}
