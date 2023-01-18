struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() <= 1 {
            return Vec::new();
        }
        use std::cmp::Ordering;
        let (mut l, mut r) = (nums.iter(), nums.iter().rev());
        let (mut lv, mut rv) = (l.next().unwrap(), r.next().unwrap());
        loop {
            match (lv + rv).cmp(&target) {
                Ordering::Equal => return vec![*lv, *rv],
                Ordering::Less => match l.next() {
                    Some(val) => lv = val,
                    _ => break,
                },
                Ordering::Greater => match r.next() {
                    Some(val) => rv = val,
                    _ => break,
                },
            }
        }
        Vec::new()
    }
}
