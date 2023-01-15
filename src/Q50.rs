pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> char {
        use std::collections::HashMap;
        let mut hm = HashMap::new();
        for ch in s.chars() {
            hm.entry(ch).and_modify(|count| *count += 1).or_insert(1);
        }
        for ch in s.chars() {
            if let Some(1) = hm.get(&ch) {
                return ch;
            }
        }
        ' '
    }
}
