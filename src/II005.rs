struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut masks = vec![];
        for word in &words {
            let mut temp = 0;
            for c in word.bytes() {
                temp |= 1 << (c - 'a' as u8);
            }
            masks.push(temp);
        }
        let mut ret = 0;
        use std::cmp::max;
        for (i, mask1) in masks.iter().enumerate() {
            for (j, mask2) in masks.iter().enumerate() {
                if mask1 & mask2 == 0 {
                    ret = max(words[i].len() * words[j].len(), ret);
                }
            }
        }
        ret as i32
    }
}
