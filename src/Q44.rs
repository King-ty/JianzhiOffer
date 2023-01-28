struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n as i64;
        let (mut pre, mut wei) = (9, 1);
        while n > pre * wei {
            n -= pre * wei;
            pre *= 10;
            wei += 1;
        }
        let num = (n - 1) / wei + 10i64.pow(wei as u32 - 1);
        let temp = wei - (n - 1) % wei;
        let temp = 10i64.pow(temp as u32);
        (num % temp / (temp / 10)) as i32
    }
}
