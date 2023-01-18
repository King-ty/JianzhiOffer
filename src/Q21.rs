struct Solution;

impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        let mut odds = Vec::new();
        let mut evens = Vec::new();
        for num in nums {
            if num & 1 == 1 {
                odds.push(num);
            } else {
                evens.push(num);
            }
        }
        odds.append(&mut evens);
        odds
    }
}
