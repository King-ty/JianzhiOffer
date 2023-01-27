// leetcode用不了
struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        use std::collections::BTreeSet;
        let mut bset = BTreeSet::new();
        bset.insert(1u64);
        let mut cur = 0u64;
        for _ in 0..n {
            // BTreeSet的first和back方法在leetcode上不稳定
            cur = bset.pop_first().unwrap();
            // cur = *bset.first().unwrap();
            // bset.take(&cur);
            // dbg!(cur);
            bset.insert(cur * 2);
            bset.insert(cur * 3);
            bset.insert(cur * 5);
        }
        cur as i32
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q49() {
        use super::Solution;
        assert_eq!(12, dbg!(Solution::nth_ugly_number(10)));
    }
}
