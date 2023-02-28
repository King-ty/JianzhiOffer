struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn next_permutation(nums: &mut Vec<i32>) -> bool {
            if nums.len() <= 1 {
                return false;
            }
            let mut i = nums.len() - 2;
            while nums[i] >= nums[i + 1] {
                if i == 0 {
                    return false;
                }
                i -= 1;
            }
            let mut j = nums.len() - 1;
            while nums[j] <= nums[i] {
                j -= 1;
            }
            nums.swap(i, j);
            i += 1;
            j = nums.len() - 1;
            while i < j {
                nums.swap(i, j);
                i += 1;
                j -= 1;
            }
            true
        }
        let mut res = vec![];
        nums.sort();
        res.push(nums.clone());
        while next_permutation(&mut nums) {
            res.push(nums.clone());
        }
        res
    }
}
