use core::num;

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        fn get_res(nums: &[i32]) -> i32 {
            let mut prev = [0; 2];
            for num in nums {
                let temp = [max(prev[0], prev[1]), prev[0] + num];
                prev = temp;
            }
            max(prev[0], prev[1])
        }
        if nums.len() == 1 {
            return nums[0];
        }
        max(get_res(&nums[1..]), get_res(&nums[..nums.len() - 1]))
    }
}

// impl Solution {
//     pub fn rob(nums: Vec<i32>) -> i32 {
//         if nums.len() == 1 {
//             return nums[0];
//         }
//         use std::cmp::max;
//         let mut prev = [0; 2];
//         for num in nums.iter().skip(1) {
//             let temp = [max(prev[0], prev[1]), prev[0] + num];
//             prev = temp;
//         }
//         let res = max(prev[0], prev[1]);
//         prev = [0; 2];
//         for num in nums.iter().into_iter().rev().skip(1) {
//             let temp = [max(prev[0], prev[1]), prev[0] + num];
//             prev = temp;
//         }
//         max(res, max(prev[0], prev[1]))
//     }
// }
