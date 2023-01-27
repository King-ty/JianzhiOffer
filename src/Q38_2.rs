struct Solution;

impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        fn dfs(ret: &mut Vec<String>, bytes: &mut Vec<u8>, ii: usize) {
            use std::collections::HashSet;
            if ii == bytes.len() - 1 {
                ret.push(String::from_utf8(bytes.to_owned()).unwrap());
                return;
            }
            let mut st = HashSet::new();
            for i in ii..bytes.len() {
                if st.contains(&bytes[i]) {
                    continue;
                }
                st.insert(bytes[i]);
                bytes.swap(ii, i);
                dfs(ret, bytes, ii + 1);
                bytes.swap(ii, i);
            }
        }
        let mut bytes = s.into_bytes();
        let mut ret = Vec::new();
        dfs(&mut ret, &mut bytes, 0);
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
