struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut cnts = [0; 26];
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut flag = false;
        for (i, j) in s.iter().zip(t.iter()) {
            if i != j {
                flag = true;
            }
            cnts[(i - b'a') as usize] -= 1;
            cnts[(j - b'a') as usize] += 1;
        }
        if !flag {
            return false;
        }
        for cnt in cnts {
            if cnt != 0 {
                return false;
            }
        }
        true
    }
}
