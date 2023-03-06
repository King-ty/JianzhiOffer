struct Solution;

// 看来我真的不太擅长这种类型的题！！！
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        use std::collections::{HashMap, HashSet, LinkedList};

        fn get_cur_str(cur: Vec<u8>) -> String {
            String::from_utf8(cur).unwrap()
        }

        // let deadends = HashSet::from_iter(deadends.into_iter());
        if target == "0000".to_string() {
            return 0;
        }
        let mut deadends_hs = HashSet::new();
        for item in deadends {
            deadends_hs.insert(item);
        }
        let deadends = deadends_hs;
        if deadends.contains(&"0000".to_string()) {
            return -1;
        }
        let mut vis = HashMap::new();
        let mut qq = LinkedList::new();
        qq.push_back("0000".to_string());
        vis.insert("0000".to_string(), 0);
        while let Some(cur) = qq.pop_front() {
            let cnt = vis[&cur];
            let mut cur = Vec::from(cur.as_bytes());
            for i in 0..4 {
                cur[i] += 1;
                if cur[i] > b'9' {
                    cur[i] -= 10;
                }
                let cur_str = get_cur_str(cur.clone());
                if cur_str == *target {
                    return cnt + 1;
                }
                if !deadends.contains(&cur_str) && !vis.contains_key(&cur_str) {
                    qq.push_back(cur_str.clone());
                    vis.insert(cur_str, cnt + 1);
                }
                cur[i] += 8;
                if cur[i] > b'9' {
                    cur[i] -= 10;
                }
                let cur_str = get_cur_str(cur.clone());
                if cur_str == *target {
                    return cnt + 1;
                }
                if !deadends.contains(&cur_str) && !vis.contains_key(&cur_str) {
                    qq.push_back(cur_str.clone());
                    vis.insert(cur_str, cnt + 1);
                }
                cur[i] += 1;
                if cur[i] > b'9' {
                    cur[i] -= 10;
                }
            }
        }
        -1
    }
}

// impl Solution {
//     pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
//         use std::cmp;
//         use std::collections::HashSet;
//         // TODO：深搜一定爆栈，尝试改广搜！
//         fn dfs(
//             deadends: &HashSet<String>,
//             target: &String,
//             cur: &mut Vec<u8>,
//             vis: &mut HashSet<String>,
//         ) -> i32 {
//             fn get_cur_str(mut cur: Vec<u8>) -> String {
//                 for i in &mut cur {
//                     *i += b'0';
//                 }
//                 String::from_utf8(cur).unwrap()
//             }
//             let mut res = i32::MAX / 2;
//             let cur_str = get_cur_str(cur.clone());
//             println!("{}", cur_str); // debug
//             vis.insert(cur_str);
//             for i in 0..4 {
//                 cur[i] = (cur[i] + 1) % 10;
//                 let cur_str = get_cur_str(cur.clone());
//                 if cur_str == *target {
//                     return 0;
//                 }
//                 if !deadends.contains(&cur_str) && !vis.contains(&cur_str) {
//                     res = cmp::min(res, dfs(deadends, target, cur, vis));
//                 }
//                 cur[i] = (cur[i] + 8) % 10;
//                 let cur_str = get_cur_str(cur.clone());
//                 if cur_str == *target {
//                     return 0;
//                 }
//                 if !deadends.contains(&cur_str) && !vis.contains(&cur_str) {
//                     res = cmp::min(res, dfs(deadends, target, cur, vis));
//                 }
//                 cur[i] = (cur[i] + 1) % 10;
//             }
//             res + 1
//         }

//         // let deadends = HashSet::from_iter(deadends.into_iter());
//         let mut deadends_hs = HashSet::new();
//         for item in deadends {
//             deadends_hs.insert(item);
//         }
//         let res = dfs(
//             &deadends_hs,
//             &target,
//             &mut vec![0, 0, 0, 0],
//             &mut HashSet::new(),
//         );
//         if res >= i32::MAX / 2 {
//             -1
//         } else {
//             res
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II109() {
        assert_eq!(
            6,
            dbg!(Solution::open_lock(
                vec![
                    "0201".to_string(),
                    "0101".to_string(),
                    "0102".to_string(),
                    "1212".to_string(),
                    "2002".to_string()
                ],
                "0202".to_string()
            ))
        )
    }
}
