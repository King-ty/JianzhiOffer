struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        fn reverse(nums: &mut Vec<i32>, mut i: usize, mut j: usize) {
            while i < j {
                nums.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        let n = nums.len();
        if n < 2 {
            return;
        }
        let mut i = n - 2;
        while nums[i] >= nums[i + 1] {
            if i == 0 {
                reverse(nums, 0, n - 1);
                return;
            }
            i -= 1;
        }
        let mut j = n - 1;
        while nums[j] <= nums[i] {
            j -= 1;
        }
        nums.swap(i, j);
        reverse(nums, i + 1, n - 1);
    }
}
