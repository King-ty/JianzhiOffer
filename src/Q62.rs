struct Solution;

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        let mut ret = 0;
        for i in 2..n + 1 {
            ret = (ret + m) % i;
        }
        ret
    }
}
