struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let (mut x, mut n) = (x, n as i64);
        let mut flag = false;
        if n < 0 {
            flag = true;
            n = -n;
        }
        let mut ret = 1.0;
        while n > 0 {
            if n & 1 == 1 {
                ret *= x;
            }
            x *= x;
            n >>= 1;
        }
        if flag {
            1.0 / ret
        } else {
            ret
        }
    }
}
