struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut qq: VecDeque<(i32, usize)> = VecDeque::new();
        let mut ret = Vec::new();
        for (i, num) in nums.into_iter().enumerate() {
            while !qq.is_empty() && qq.back().unwrap().0 <= num {
                qq.pop_back();
            }
            qq.push_back((num, i));
            if i < (k - 1) as usize {
                continue;
            }
            // println!("{:?} {:?}", qq, qq.front().unwrap().1);
            while qq.front().unwrap().1 < i - k as usize + 1 {
                qq.pop_front();
            }
            ret.push(qq.front().unwrap().0);
        }
        ret
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q59i() {
        use super::Solution;
        assert_eq!(
            vec![3, 3, 5, 5, 6, 7],
            dbg!(Solution::max_sliding_window(
                vec![1, 3, -1, -3, 5, 3, 6, 7],
                3
            ))
        );
    }
}
