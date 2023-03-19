struct Solution;

impl Solution {
    pub fn search_range(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn lower_bound(nums: &mut Vec<i32>, target: i32) -> usize {
            let (mut l, mut r) = (0, nums.len());
            while l < r {
                let mid = (l + r) / 2;
                if nums[mid] < target {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            l
        }
        let l = lower_bound(&mut nums, target);
        if l == nums.len() || nums[l] != target {
            vec![-1, -1]
        } else {
            vec![l as i32, lower_bound(&mut nums, target + 1) as i32 - 1]
        }
    }
}
