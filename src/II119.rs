struct Solution;

// 看起来更好的解法：只从连续区间的起点开始扩展
// 但是不知道为啥表现得非常慢，不理解
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let exists: HashSet<i32> = nums.clone().into_iter().collect();
        let mut res = 0;
        for num in nums {
            // 以下判断可以保证是连续区间的起点
            // if exists.get(&(num - 1)).is_none() {
            if !exists.contains(&(num - 1)) {
                let mut cur_len = 1;
                let mut cur = num + 1;
                // while exists.get(&cur).is_some() {
                while exists.contains(&cur) {
                    cur += 1;
                    cur_len += 1;
                }
                res = res.max(cur_len);
            }
        }
        res
    }
}

// 使用哈希表vis来记忆化搜索
impl Solution {
    pub fn longest_consecutive_mine(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let exists: HashSet<i32> = nums.clone().into_iter().collect();
        let mut vis: HashSet<i32> = HashSet::new();
        let mut res = 0;
        for num in nums {
            if !vis.contains(&num) {
                vis.insert(num);
                let mut cur_len = 1;
                let mut cur = num + 1;
                while exists.contains(&cur) {
                    vis.insert(cur);
                    cur += 1;
                    cur_len += 1;
                }
                cur = num - 1;
                while exists.contains(&cur) {
                    vis.insert(cur);
                    cur -= 1;
                    cur_len += 1;
                }
                res = res.max(cur_len);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II119() {
        assert_eq!(
            4,
            dbg!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]))
        );
    }
}
