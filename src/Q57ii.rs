// 双指针滑窗
struct Solution;

impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let n = (target + 1) / 2;
        let mut ret = Vec::new();
        let (mut l, mut r) = (1, 2);
        while r <= n {
            let ss = (r + l) * (r - l + 1) / 2;
            if ss == target {
                ret.push((l..=r).collect());
                l += 1;
                r += 1;
            } else if ss > target {
                l += 1;
            } else {
                r += 1;
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q57ii() {
        use super::Solution;
        let ret = Solution::find_continuous_sequence(9);
        assert_eq!(vec![vec![2, 3, 4], vec![4, 5]], dbg!(ret));
    }
}
