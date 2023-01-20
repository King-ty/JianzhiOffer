use std::collections::LinkedList;

struct Solution;

impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        fn check(mut x: i32, mut y: i32, k: i32) -> bool {
            let mut s = 0;
            while x > 0 {
                s += x % 10;
                x /= 10;
            }
            while y > 0 {
                s += y % 10;
                y /= 10;
            }
            s <= k
        }
        use std::collections::LinkedList;
        const MV: [[i32; 2]; 2] = [[1, 0], [0, 1]];
        let mut ret = 1;
        let mut vs = vec![vec![false; n as usize]; m as usize];
        let mut qq = LinkedList::new();
        qq.push_back((0, 0));
        vs[0][0] = true;
        while !qq.is_empty() {
            let (x, y) = qq.pop_front().unwrap();

            for mv in MV {
                if x + mv[0] < m
                    && x + mv[0] >= 0
                    && y + mv[1] < n
                    && y + mv[1] >= 0
                    && !vs[(x + mv[0]) as usize][(y + mv[1]) as usize]
                {
                    if check(x + mv[0], y + mv[1], k) {
                        qq.push_back((x + mv[0], y + mv[1]));
                        vs[(x + mv[0]) as usize][(y + mv[1]) as usize] = true;
                        ret += 1;
                    }
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q13() {
        use super::Solution;
        assert_eq!(7, Solution::moving_count(7, 2, 3));
    }
}
