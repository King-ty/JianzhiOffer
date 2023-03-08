struct Solution;

// 双向广搜
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        use std::collections::{HashMap, LinkedList};
        fn bfs(begin: &String, end: &String, graph: &HashMap<String, Vec<(String, f64)>>) -> f64 {
            let mut begin_q = LinkedList::new();
            let mut end_q = LinkedList::new();
            let mut begin_seen = HashMap::new();
            let mut end_seen = HashMap::new();
            begin_q.push_back((begin, 1.0));
            begin_seen.insert(begin, 1.0);
            end_q.push_back((end, 1.0));
            end_seen.insert(end, 1.0);
            while !begin_q.is_empty() && !end_q.is_empty() {
                let (cur, cal) = begin_q.pop_front().unwrap();
                for (to, fac) in &graph[cur] {
                    if to == end {
                        return cal * fac;
                    }
                    if end_seen.contains_key(to) {
                        return cal * fac / end_seen[to];
                    }
                    if !begin_seen.contains_key(to) {
                        begin_q.push_back((to, cal * fac));
                        begin_seen.insert(to, cal * fac);
                    }
                }
                let (cur, cal) = end_q.pop_front().unwrap();
                for (to, fac) in &graph[cur] {
                    if to == begin {
                        return 1.0 / (cal * fac);
                    }
                    if begin_seen.contains_key(to) {
                        return begin_seen[to] / (cal * fac);
                    }
                    if !end_seen.contains_key(to) {
                        end_q.push_back((to, cal * fac));
                        end_seen.insert(to, cal * fac);
                    }
                }
            }
            -1.0
        }

        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        for (equation, value) in equations.into_iter().zip(values) {
            graph
                .entry(equation[0].clone())
                .or_insert(vec![])
                .push((equation[1].clone(), value));
            graph
                .entry(equation[1].clone())
                .or_insert(vec![])
                .push((equation[0].clone(), 1.0 / value));
        }
        let mut res = vec![];
        for query in queries {
            if graph.get(&query[0]).is_none() || graph.get(&query[1]).is_none() {
                res.push(-1.0);
            } else if query[0] == query[1] {
                res.push(1.0);
            } else {
                res.push(bfs(&query[0], &query[1], &graph));
            }
        }
        res
    }
}
