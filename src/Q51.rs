// 归并法
struct Solution;

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        fn merge_sort(l: usize, r: usize, nums: &mut Vec<i32>) -> i32 {
            if l >= r - 1 {
                return 0;
            }
            let mid = (l + r) / 2;
            let mut ret = merge_sort(l, mid, nums) + merge_sort(mid, r, nums);
            let (mut i, mut j) = (l, mid);
            let mut temp = Vec::new();
            while i < mid && j < r {
                if nums[i] <= nums[j] {
                    temp.push(nums[i]);
                    i += 1;
                } else {
                    temp.push(nums[j]);
                    ret += (mid - i) as i32;
                    j += 1;
                }
            }
            while i < mid {
                temp.push(nums[i]);
                i += 1;
            }
            while j < r {
                temp.push(nums[j]);
                j += 1;
            }
            for i in l..r {
                nums[i] = temp[i - l];
            }
            ret
        }
        if nums.is_empty() {
            return 0;
        }
        let mut nums = nums;
        merge_sort(0, nums.len(), &mut nums)
    }
}
