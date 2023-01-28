struct Solution;

impl Solution {
    pub fn print_numbers(n: i32) -> Vec<i32> {
        Solution::print_numbers_string(n)
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect()
    }

    fn print_numbers_string(n: i32) -> Vec<String> {
        fn dfs(nn: i32, ret: &mut Vec<String>, num: &mut String) {
            if nn == 0 {
                let val = strip(num);
                if val != "" {
                    ret.push(val);
                }
                return;
            }
            for i in 0..=9 {
                num.push((i as u8 + '0' as u8) as char);
                dfs(nn - 1, ret, num);
                num.pop();
            }
        }
        fn strip(s: &mut String) -> String {
            let mut i = 0;
            let bytes = s.as_bytes();
            while i < s.len() && bytes[i] == '0' as u8 {
                i += 1;
            }
            if i == s.len() {
                "".to_string()
            } else {
                s[i..].to_owned()
            }
        }
        let mut ret = Vec::new();
        dfs(n, &mut ret, &mut "".to_string());
        ret
    }
}
