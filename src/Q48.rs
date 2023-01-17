struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        let mut exist = vec![false; 128];
        let mut l = 0;
        let mut r = 0;
        let bytes = s.as_bytes();
        let mut ret = 0;
        for c in bytes {
            while exist[*c as usize] {
                exist[bytes[l as usize] as usize] = false;
                l += 1;
            }
            exist[*c as usize] = true;
            r += 1;
            ret = max(ret, r - l);
        }
        ret
    }
}
