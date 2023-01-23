struct Solution;

impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }
        let (a, b) = ((n / 3) as u32, n % 3);
        if b == 1 {
            3i32.pow(a - 1) * 4
        } else if b == 0 {
            3i32.pow(a)
        } else {
            3i32.pow(a) * 2
        }
    }
}
