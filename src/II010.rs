struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut ret = 0;
        let mut sm = 0;
        let mut tms = HashMap::new();
        tms.insert(0, 1);
        for num in nums {
            sm += num;
            if let Some(val) = tms.get(&(sm - k)) {
                ret += val;
            }
            *tms.entry(sm).or_insert(0) += 1;
        }
        ret
    }
}
