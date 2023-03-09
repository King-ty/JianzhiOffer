struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut exists = [false; 128];
        let bytes = s.as_bytes();
        let mut res = 0;
        for r in 0..bytes.len() {
            while exists[bytes[r] as usize] {
                exists[bytes[l] as usize] = false;
                l += 1;
            }
            exists[bytes[r] as usize] = true;
            res = res.max(r - l + 1);
        }
        res as i32
    }
}
