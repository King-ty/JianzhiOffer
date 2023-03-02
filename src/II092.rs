struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        use std::cmp::min;
        let mut res = [0, 0];
        for bt in s.bytes() {
            let is1 = if bt == b'0' { 0 } else { 1 };
            res = [res[0] + is1, min(res[0], res[1]) + 1 - is1]
        }
        min(res[0], res[1])
    }
}
