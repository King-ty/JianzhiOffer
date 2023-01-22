struct Solution;

impl Solution {
    pub fn add(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a, b);
        while b != 0 {
            let up = (a & b) << 1;
            a ^= b;
            b = up;
        }
        a
    }
}
