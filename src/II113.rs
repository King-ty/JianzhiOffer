struct Solution;

// 拓扑排序
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        use std::{collections::LinkedList, vec};
        let num_courses = num_courses as usize;
        let mut graph = vec![vec![]; num_courses];
        let mut in_num = vec![0; num_courses];
        for prerequisite in prerequisites {
            let (y, x) = (prerequisite[0] as usize, prerequisite[1] as usize);
            graph[x].push(y);
            in_num[y] += 1;
        }
        let mut qq = LinkedList::new();
        for i in 0..num_courses {
            if in_num[i] == 0 {
                qq.push_back(i);
            }
        }
        let mut ret = vec![];
        while let Some(cur) = qq.pop_front() {
            ret.push(cur as i32);
            for &to in &graph[cur] {
                in_num[to] -= 1;
                if in_num[to] == 0 {
                    qq.push_back(to);
                }
            }
        }
        if ret.len() < num_courses {
            vec![]
        } else {
            ret
        }
    }
}
