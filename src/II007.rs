struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        let mut nums = nums;
        nums.sort();
        let mut ret = vec![];
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            while j < k {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    j += 1;
                    continue;
                }
                if k < nums.len() - 1 && nums[k] == nums[k + 1] {
                    k -= 1;
                    continue;
                }
                match (nums[j] + nums[k]).cmp(&-nums[i]) {
                    Ordering::Equal => {
                        ret.push(vec![nums[i], nums[j], nums[k]]);
                        j += 1;
                        k -= 1;
                    }
                    Ordering::Less => j += 1,
                    Ordering::Greater => k -= 1,
                }
            }
        }
        ret
    }
}
