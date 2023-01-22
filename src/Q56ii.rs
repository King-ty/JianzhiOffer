struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut tms = Vec::with_capacity(32);
        tms.resize(32, 0);
        for mut num in nums {
            let mut i = 0;
            while num > 0 {
                tms[i] += num & 1;
                num >>= 1;
                i += 1;
            }
        }
        let mut ret = 0;
        for i in 0..32 {
            if tms[i] % 3 == 1 {
                ret |= 1 << i;
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q56ii() {
        use super::Solution;
        assert_eq!(4, Solution::single_number(vec![3, 4, 3, 3]));
    }
}
