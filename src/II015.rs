struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (m, n) = (s.len(), p.len());
        let mut ret = vec![];
        if n > m {
            return ret;
        }
        let mut cnt = [0; 26];
        let (s, p) = (s.as_bytes(), p.as_bytes());
        for c in p {
            cnt[(c - b'a') as usize] -= 1;
        }
        let mut l = 0;
        for r in 0..m {
            let x = (s[r] - b'a') as usize;
            cnt[x] += 1;
            while cnt[x] > 0 {
                let y = (s[l] - b'a') as usize;
                cnt[y] -= 1;
                l += 1;
            }
            if r - l + 1 == n {
                ret.push(l as i32);
            }
        }
        ret
    }
}
