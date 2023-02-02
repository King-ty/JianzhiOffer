struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);
        for num in nums {
            a = !b & (a ^ num);
            b = !a & (b ^ num);
        }
        a
    }
}
