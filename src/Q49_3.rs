struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        use std::cmp::min;
        let mut v = vec![1];
        let (mut p2, mut p3, mut p5) = (0, 0, 0);
        let mut nw = 1;
        for _ in 1..n {
            nw = min(min(v[p2] * 2, v[p3] * 3), v[p5] * 5);
            v.push(nw);
            if v[p2] * 2 == nw {
                p2 += 1;
            }
            if v[p3] * 3 == nw {
                p3 += 1;
            }
            if v[p5] * 5 == nw {
                p5 += 1;
            }
        }
        nw
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
