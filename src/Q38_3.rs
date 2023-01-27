struct Solution;

impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        fn next_permutation(bytes: &mut Vec<u8>) -> bool {
            if bytes.len() <= 1 {
                return false;
            }
            let mut i = bytes.len() - 2;
            while bytes[i] >= bytes[i + 1] {
                if i == 0 {
                    return false;
                }
                i -= 1;
            }
            let mut j = bytes.len() - 1;
            while bytes[j] <= bytes[i] {
                j -= 1;
            }
            bytes.swap(i, j);
            i += 1;
            j = bytes.len() - 1;
            while i < j {
                bytes.swap(i, j);
                i += 1;
                j -= 1;
            }
            true
        }
        let mut bytes = s.into_bytes();
        bytes.sort();
        let mut ret = Vec::new();
        loop {
            ret.push(String::from_utf8(bytes.clone()).unwrap());
            if !next_permutation(&mut bytes) {
                break;
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q38() {
        use super::Solution;
        let mut ret = Solution::permutation("aabc".to_string());
        ret.sort();
        assert_eq!(vec!["suvyls"], dbg!(ret));
    }
}
