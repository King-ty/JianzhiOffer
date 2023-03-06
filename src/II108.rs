struct Solution;

// 朴素建图+广搜做法。
// 这里建图复杂度O(n^2)，可以通过添加"h*t"这样的虚拟节点来讲复杂度降为O(n*len)
// 另外可以使用双向搜索优化降低常数
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        use std::collections::LinkedList;
        fn is_connect(s1: &str, s2: &str) -> bool {
            let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
            let n = s1.len();
            if n != s2.len() {
                return false;
            }
            let mut diff_flag = false;
            for i in 0..n {
                if s1[i] != s2[i] {
                    if diff_flag {
                        return false;
                    }
                    diff_flag = true;
                }
            }
            true
        }
        let mut n = word_list.len();
        let mut edges = vec![vec![]; n];
        let mut begin_index = n + 1;
        let mut end_index = n + 1;
        // 建图
        for i in 0..n {
            let word = &word_list[i];
            if *word == begin_word {
                begin_index = i;
            } else if *word == end_word {
                end_index = i;
            }
            for j in 0..i {
                let other = &word_list[j];
                if is_connect(word, other) {
                    edges[i].push(j);
                    edges[j].push(i);
                }
            }
        }
        if end_index == n + 1 {
            return 0;
        }
        if begin_index == n + 1 {
            word_list.push(begin_word);
            edges.push(vec![]);
            begin_index -= 1;
            n += 1;
            let word = &word_list[begin_index];
            for j in 0..n {
                let other = &word_list[j];
                if is_connect(word, other) {
                    edges[begin_index].push(j);
                    edges[j].push(begin_index);
                }
            }
        }

        // n = dbg!(n);
        // begin_index = dbg!(begin_index);
        // end_index = dbg!(end_index);
        // edges = dbg!(edges);

        // 深搜
        let mut dis = vec![n + 1; n];
        dis[begin_index] = 1;
        let mut qq = LinkedList::new();
        qq.push_back(begin_index);
        while let Some(cur) = qq.pop_front() {
            for &to in &edges[cur] {
                if to == end_index {
                    return dis[cur] as i32 + 1;
                }
                if dis[to] == n + 1 {
                    dis[to] = dis[cur] + 1;
                    qq.push_back(to);
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II108() {
        assert_eq!(
            5,
            dbg!(Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            ))
        )
    }
}
