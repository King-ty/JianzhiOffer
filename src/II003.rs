struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ret = vec![0; n as usize + 1];
        for i in 1..=n as usize {
            ret[i] = ret[i & (i - 1)] + 1;
        }
        ret
    }
}
