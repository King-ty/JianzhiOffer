struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(
            graph: &Vec<Vec<i32>>,
            res: &mut Vec<Vec<i32>>,
            path: &mut Vec<i32>,
            cur: usize,
            goal: usize,
        ) {
            path.push(cur as i32);
            if cur == goal {
                res.push(path.clone());
            }
            for to in &graph[cur] {
                dfs(graph, res, path, *to as usize, goal);
            }
            path.pop();
        }
        let goal = graph.len() - 1;
        let mut res = vec![];
        dfs(&graph, &mut res, &mut vec![], 0, goal);
        res
    }
}
