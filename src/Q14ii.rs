struct Solution;

impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        const MOD: u32 = 1000000007;
        if n <= 3 {
            return n - 1;
        }
        let (a, b) = ((n / 3) as u32, n % 3);
        let mut ret = 1;
        for _ in 1..a {
            ret = (ret * 3) % MOD;
        }
        if b == 1 {
            (ret * 4 % MOD) as i32
        } else if b == 0 {
            (ret * 3 % MOD) as i32
        } else {
            (ret * 3 % MOD * 2 % MOD) as i32
        }
    }
}
