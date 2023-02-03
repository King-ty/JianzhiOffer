struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        use std::collections::HashMap;
        let mut mp = HashMap::new();
        mp.insert(0, 0);
        let mut sm = 0;
        let mut ret = 0;
        for (i, num) in nums.into_iter().enumerate() {
            sm += if num == 1 { 1 } else { -1 };
            mp.entry(sm).or_insert(i + 1);
            if let Some(val) = mp.get(&sm) {
                ret = max(ret, i + 1 - val);
            }
        }
        ret as i32
    }
}
