struct Solution;

// 类似题解方法
impl Solution {
    pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
        use std::collections::{HashSet, LinkedList};
        let n = nums.len();
        let mut graph = vec![vec![]; n];
        let mut in_degree = vec![0; n];
        // 建图
        for sequence in sequences {
            for window in sequence.windows(2) {
                let (pre, cur) = (window[0] as usize - 1, window[1] as usize - 1);
                in_degree[cur] += 1;
                graph[pre].push(cur);
            }
        }
        // 拓扑排序
        let mut qq = LinkedList::new();
        for i in 0..n {
            if in_degree[i] == 0 {
                qq.push_back(i);
                if qq.len() > 1 {
                    return false;
                }
            }
        }
        let mut res = vec![];
        while let Some(cur) = qq.pop_front() {
            res.push(cur as i32 + 1);
            for &to in &graph[cur] {
                in_degree[to] -= 1;
                if in_degree[to] == 0 {
                    qq.push_back(to);
                    if qq.len() > 1 {
                        return false;
                    }
                }
            }
        }
        res == nums
    }
}

// 不如题解优雅
impl Solution {
    pub fn sequence_reconstruction_mine(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
        use std::collections::{HashSet, LinkedList};
        let n = nums.len();
        let mut graph = vec![HashSet::new(); n + 1];
        let mut in_degree = vec![0; n + 1];
        let mut seen = vec![false; n + 1];
        sequences
            .iter()
            .flatten()
            .for_each(|x| seen[*x as usize] = true);
        let seen_num = seen.iter().filter(|x| **x).count();
        if seen_num != nums.len() {
            return false;
        }
        for sequence in sequences {
            for (pre, cur) in sequence.iter().zip(sequence.iter().skip(1)) {
                let (pre, cur) = (*pre as usize, *cur as usize);
                if !graph[pre].contains(&cur) {
                    in_degree[cur] += 1;
                    graph[pre].insert(cur);
                }
            }
        }
        let mut qq = LinkedList::new();
        for i in 1..n + 1 {
            if in_degree[i] == 0 {
                qq.push_back(i);
            }
        }
        if qq.len() != 1 {
            return false;
        }
        let mut ans = vec![];
        while let Some(cur) = qq.pop_front() {
            ans.push(cur as i32);
            for &to in &graph[cur] {
                in_degree[to] -= 1;
                if in_degree[to] == 0 {
                    qq.push_back(to);
                }
            }
            if qq.len() > 1 {
                return false;
            }
        }
        if ans.len() < seen_num {
            return false;
        }
        for i in 0..seen_num {
            if nums[i] != ans[i] {
                return false;
            }
        }
        true
    }
}
