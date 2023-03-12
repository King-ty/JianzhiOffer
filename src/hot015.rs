struct Solution;

// 排序+双指针
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let n = nums.len();
        let mut res = vec![];
        for i in 0..n - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                if l > i + 1 && nums[l] == nums[l - 1] {
                    l += 1;
                    continue;
                }
                if r < n - 1 && nums[r] == nums[r + 1] {
                    r -= 1;
                    continue;
                }
                if nums[l] + nums[r] == -nums[i] {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                } else if nums[l] + nums[r] < -nums[i] {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        res
    }
}

// 哈希法,实际上是O(n^2)的复杂度，但是比较慢
impl Solution {
    pub fn three_sum_mine(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        nums.sort();
        let n = nums.len();
        let mut res = HashSet::new();
        let mut exists = HashSet::new();
        for i in 0..n {
            for j in (i + 1)..n {
                if exists.contains(&-(nums[i] + nums[j])) {
                    res.insert(vec![-(nums[i] + nums[j]), nums[i], nums[j]]);
                }
            }
            exists.insert(nums[i]);
        }
        res.into_iter().collect()
    }
}
