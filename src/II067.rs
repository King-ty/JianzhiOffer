struct Solution;

// 原理：x=a^b => a=x^b
impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        const HIGH_BIT: i32 = 30;
        let mut res = 0;
        for i in (0..=HIGH_BIT).rev() {
            let mut hs = HashSet::new();
            let temp = res * 2 + 1;
            res <<= 1;
            for num in &nums {
                let num = *num >> i;
                if hs.contains(&(num ^ temp)) {
                    res = temp;
                    break;
                }
                hs.insert(num);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II067() {
        dbg!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]));
    }
}
