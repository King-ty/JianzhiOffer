// 优化空间
struct Solution;

impl Solution {
    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        let len = a.len();
        if len == 0 {
            return Vec::new();
        }
        let mut ret = Vec::with_capacity(len);
        ret.resize(len, 1);
        let mut ss = 1;
        for i in 0..(len - 1) {
            ss *= a[i];
            ret[i + 1] = ss;
        }
        ss = 1;
        for i in (0..len).rev() {
            ret[i] *= ss;
            ss *= a[i];
        }
        ret
    }
}
