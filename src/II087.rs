pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn get_res<'a>(s: &'a str, i_start: usize, cur: &mut Vec<&'a str>, res: &mut Vec<String>) {
            let n = s.len();
            if cur.len() == 4 {
                if i_start == n {
                    res.push(cur.join("."));
                }
                return;
            }
            let mut i = i_start;
            while i < n && i - i_start < 3 {
                if s.as_bytes()[i_start] == b'0' && i > i_start {
                    break;
                }
                if s[i_start..=i].parse::<i32>().unwrap() <= 255 {
                    cur.push(&s[i_start..=i]);
                    get_res(s, i + 1, cur, res);
                    cur.pop();
                }
                i += 1;
            }
        }
        let mut res = vec![];
        get_res(&s, 0, &mut vec![], &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        dbg!(Solution::restore_ip_addresses("25525511135".to_string()));
    }
}
