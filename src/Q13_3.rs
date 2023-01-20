// 这方法不行，必须搜索
struct Solution;

impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut ret = 0;
        let mut cur_s;
        let mut la = 8;
        for i in 0..m {
            if i % 10 == 0 {
                cur_s = la - 8;
            } else {
                cur_s = la + 1;
            }
            la = cur_s;
            if cur_s > k {
                break;
            }
            for j in 0..n {
                if cur_s <= k {
                    ret += 1;
                    if j % 10 == 9 {
                        cur_s -= 8;
                    } else {
                        cur_s += 1;
                    }
                } else {
                    break;
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
        let ret = Solution::moving_count(38, 15, 9);
        println!("{}", ret);
        assert_eq!(135, ret);
    }
}
