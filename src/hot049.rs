struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        fn get_cnts(s: &str) -> [i32; 26] {
            let mut ret = [0; 26];
            for &byte in s.as_bytes() {
                ret[(byte - b'a') as usize] += 1;
            }
            ret
        }
        let mut mp = HashMap::new();
        let mut ret: Vec<Vec<String>> = vec![];
        for s in strs {
            let cnts = get_cnts(&s);
            if let Some(index) = mp.get(&cnts) {
                ret[*index as usize].push(s);
            } else {
                mp.insert(cnts, ret.len());
                ret.push(vec![s]);
            }
        }
        ret
    }
}
