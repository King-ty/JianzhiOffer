struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (ss, tt) = (s.as_bytes(), t.as_bytes());
        let mut cnt = [0; 60];
        let mut cnt_num = 0;
        for &c in tt {
            let ind = (c - b'A') as usize;
            if cnt[ind] == 0 {
                cnt_num += 1;
            }
            cnt[ind] -= 1;
        }
        let (mut min_len, mut ret) = (100009, "");
        let s_len = ss.len();
        let mut l = 0;
        for r in 0..ss.len() {
            let ind = (ss[r] - b'A') as usize;
            cnt[ind] += 1;
            if cnt[ind] == 0 {
                cnt_num -= 1;
            }
            while l < s_len && cnt[(ss[l] - b'A') as usize] > 0 {
                cnt[(ss[l] - b'A') as usize] -= 1;
                l += 1;
            }
            if cnt_num == 0 && r - l + 1 < min_len {
                min_len = r - l + 1;
                ret = &s[l..r + 1];
            }
        }

        ret.to_string()
    }
}
