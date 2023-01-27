struct Solution;

impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        fn dfs(
            ret: &mut Vec<String>,
            bytes: &mut Vec<u8>,
            s: &mut Vec<u8>,
            vis: &mut Vec<bool>,
            ii: usize,
        ) {
            if ii == bytes.len() {
                ret.push(String::from_utf8(s.clone()).unwrap());
                return;
            }
            for i in 0..bytes.len() {
                if vis[i] || (i > 0 && bytes[i] == bytes[i - 1] && !vis[i - 1]) {
                    continue;
                }
                s.push(bytes[i]);
                vis[i] = true;
                dfs(ret, bytes, s, vis, ii + 1);
                s.pop();
                vis[i] = false;
            }
        }
        let mut bytes = s.into_bytes();
        bytes.sort();
        let mut ret = Vec::new();
        let mut vis = Vec::new();
        vis.resize(bytes.len(), false);
        dfs(&mut ret, &mut bytes, &mut Vec::new(), &mut vis, 0);
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
