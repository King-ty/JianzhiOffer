struct Solution;

impl Solution {
    pub fn min_number(nums: Vec<i32>) -> String {
        let mut nums = nums
            .into_iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>();
        nums.sort_by(|a, b| {
            format!("{}{}", a, b)
                .partial_cmp(&format!("{}{}", b, a))
                .unwrap()
        });
        nums.concat()
    }
}

// [3,30,34,5,9]
