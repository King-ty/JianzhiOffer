struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        use std::cmp;
        let mut bytes = vec![b'$', b'#'];
        for c in s.as_bytes() {
            bytes.push(*c);
            bytes.push(b'#');
        }
        bytes.push(b'!');
        let n = bytes.len();
        let (mut rm, mut im) = (0, 0);
        let (mut max_len, mut max_len_index) = (0, 0);
        let mut f = vec![0; n];
        for i in 1..n - 1 {
            f[i] = if i < rm {
                cmp::min(rm - i + 1, f[im * 2 - i])
            } else {
                1
            };
            while bytes[i + f[i]] == bytes[i - f[i]] {
                f[i] += 1;
            }
            if i + f[i] - 1 > rm {
                rm = i + f[i] - 1;
                im = i;
            }
            if f[i] > max_len {
                max_len = f[i];
                max_len_index = i;
            }
        }
        s[(max_len_index - max_len + 2) / 2 - 1..(max_len_index + max_len - 2) / 2].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II005() {
        dbg!(Solution::longest_palindrome("babad".to_string()));
    }
}
