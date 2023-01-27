struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashSet};
        const FACTORS: [u64; 3] = [2, 3, 5];
        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::new();
        heap.push(Reverse(1u64));
        seen.insert(1u64);
        let mut cur = 0u64;
        for _ in 0..n {
            cur = heap.pop().unwrap().0;
            for factor in FACTORS {
                let val = cur * factor;
                match seen.get(&val) {
                    Some(_) => continue,
                    None => {
                        seen.insert(val);
                        heap.push(Reverse(val));
                    }
                }
            }
        }
        cur as i32
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q49() {
        use super::Solution;
        assert_eq!(12, dbg!(Solution::nth_ugly_number(10)));
    }
}
