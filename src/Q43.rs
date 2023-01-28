struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let n: i64 = n as i64;
        let mut i: i64 = 1;
        let mut ret = 0;
        while i <= n {
            ret += n / (i * 10) * i;
            if n % (i * 10) / i > 1 {
                ret += i;
            } else if n % (i * 10) / i == 1 {
                ret += n % i + 1;
            }
            i *= 10;
        }
        ret as i32
    }
}
