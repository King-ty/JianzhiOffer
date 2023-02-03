struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let all_sum: i32 = nums.iter().sum();
        let mut sm = 0;
        for (i, num) in nums.into_iter().enumerate() {
            if sm == all_sum - num - sm {
                return i as i32;
            }
            sm += num;
        }
        -1
    }
}
