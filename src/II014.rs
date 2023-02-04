// 自己想的滑动窗口，和题解几乎一模一样
struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (n, m) = (s1.len(), s2.len());
        if n > m {
            return false;
        }
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        let mut cnt = [0; 26];
        for i in 0..n {
            cnt[(s1[i] - b'a') as usize] += 1;
            cnt[(s2[i] - b'a') as usize] -= 1;
        }
        let mut diff = 0;
        cnt.iter().for_each(|x| {
            if *x != 0 {
                diff += 1
            }
        });
        if diff == 0 {
            return true;
        }
        for i in n..m {
            let x = (s2[i] - b'a') as usize;
            if cnt[x] == 0 {
                diff += 1;
            }
            cnt[x] -= 1;
            if cnt[x] == 0 {
                diff -= 1;
            }
            let x = (s2[i - n] - b'a') as usize;
            if cnt[x] == 0 {
                diff += 1;
            }
            cnt[x] += 1;
            if cnt[x] == 0 {
                diff -= 1;
            }
            if diff == 0 {
                return true;
            }
        }

        false
    }
}
