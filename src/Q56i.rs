struct Solution;

impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut ss = 0;
        for num in &nums {
            ss ^= *num;
        }
        let mut d = 1;
        while d & ss == 0 {
            d <<= 1;
        }
        let (mut a, mut b) = (0, 0);
        for num in nums {
            if num & d == 0 {
                a ^= num;
            } else {
                b ^= num;
            }
        }
        vec![a, b]
    }
}
