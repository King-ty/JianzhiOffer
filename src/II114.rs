struct Solution;

// 根据题解的改进
impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        use std::cmp;
        use std::collections::{HashSet, LinkedList};
        const CH_NUM: usize = 26;
        let mut graph = vec![vec![]; CH_NUM];
        let mut in_degree = [0; CH_NUM];
        let mut seen = [false; CH_NUM];
        // 统计见过的字母
        words
            .iter()
            .map(|x| x.as_bytes())
            .flatten()
            .map(|&x| (x - b'a') as usize)
            .for_each(|x| seen[x] = true);
        let seen_num = seen.iter().filter(|x| **x).count();
        // 建图
        for (pre, cur) in words
            .iter()
            .map(|x| x.as_bytes())
            .zip(words.iter().skip(1).map(|x| x.as_bytes()))
        {
            let mut flag = false;
            for k in 0..cmp::min(pre.len(), cur.len()) {
                if pre[k] != cur[k] {
                    let (x, y) = ((pre[k] - b'a') as usize, (cur[k] - b'a') as usize);
                    flag = true;
                    graph[x].push(y);
                    in_degree[y] += 1;
                    break;
                }
            }
            // 出现不合理的字典序，直接返回空字符串
            if !flag && pre.len() > cur.len() {
                return String::new();
            }
        }

        // 拓扑排序
        let mut qq = LinkedList::new();
        for i in 0..CH_NUM {
            if in_degree[i] == 0 && seen[i] {
                qq.push_back(i);
            }
        }
        let mut res = String::new();
        while let Some(cur) = qq.pop_front() {
            res.push((cur as u8 + b'a') as char);
            for &to in &graph[cur] {
                in_degree[to] -= 1;
                if in_degree[to] == 0 {
                    qq.push_back(to);
                }
            }
        }

        if res.len() < seen_num {
            String::new()
        } else {
            res
        }
    }
}

// 自己的思路，有些瑕疵
impl Solution {
    pub fn alien_order_mine(words: Vec<String>) -> String {
        use std::cmp;
        use std::collections::{HashSet, LinkedList};
        let n = words.len();
        const CH_NUM: usize = 26;
        let mut graph = vec![HashSet::new(); CH_NUM];
        let mut in_degree = [0; CH_NUM];
        let mut seen = [false; CH_NUM];
        // 建图
        for i in 0..n {
            for &x in words[i].as_bytes() {
                let x = (x - b'a') as usize;
                seen[x] = true;
            }
            for j in 0..i {
                let (w1, w2) = (words[i].as_bytes(), words[j].as_bytes());
                let mut flag = false;
                for k in 0..cmp::min(w1.len(), w2.len()) {
                    let (y, x) = ((w1[k] - b'a') as usize, (w2[k] - b'a') as usize);
                    if x != y {
                        flag = true;
                        if !graph[x].contains(&y) {
                            graph[x].insert(y);
                            in_degree[y] += 1;
                        }
                        break;
                    }
                }
                // 出现不合理的字典序，直接返回空字符串
                if !flag && w1.len() < w2.len() {
                    return "".to_string();
                }
            }
        }
        let mut seen_num = 0;
        for flag in seen {
            if flag {
                seen_num += 1;
            }
        }
        // 拓扑排序
        let mut qq = LinkedList::new();
        for i in 0..CH_NUM {
            if in_degree[i] == 0 && seen[i] {
                qq.push_back(i);
            }
        }
        let mut res = "".to_string();
        while let Some(cur) = qq.pop_front() {
            res.push((cur as u8 + b'a') as char);
            for &to in &graph[cur] {
                in_degree[to] -= 1;
                if in_degree[to] == 0 {
                    qq.push_back(to);
                }
            }
        }

        if res.len() < seen_num {
            "".to_string()
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II114() {
        // 这题细节不少，因此测试点多
        assert_eq!(
            "wertf",
            Solution::alien_order(vec![
                "wrt".to_string(),
                "wrf".to_owned(),
                "er".to_string(),
                "ett".to_string(),
                "rftt".to_string()
            ])
        );
        assert_eq!(
            "z",
            Solution::alien_order(vec!["z".to_string(), "z".to_string()])
        );
        assert_eq!(
            "abcd",
            Solution::alien_order(vec!["ab".to_string(), "adc".to_string()])
        );
        assert_eq!(
            "",
            dbg!(Solution::alien_order(vec![
                "abc".to_string(),
                "ab".to_string()
            ]))
        );
    }
}
