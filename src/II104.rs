struct Solution;

// 有顺序的多重背包？朴素多重背包两层枚举反过来
impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        nums.sort();
        dp[0] = 1;
        for j in 1..=target {
            for num in &nums {
                if *num as usize > j {
                    break;
                }
                dp[j] += dp[j - *num as usize];
            }
        }
        dp[target]
    }
}

// // 朴素多重背包
// impl Solution {
//     pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
//         let target = target as usize;
//         let mut dp = vec![0; target + 1];
//         dp[0] = 1;
//         for num in nums {
//             for j in num as usize..=target {
//                 dp[j] += dp[j - num as usize];
//             }
//         }
//         dp[target]
//     }
// }
