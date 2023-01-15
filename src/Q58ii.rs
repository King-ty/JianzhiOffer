pub struct Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let n = n as usize; // 注意必须使用usize
        format!("{}{}", &s[n..], &s[..n])
    }
}
