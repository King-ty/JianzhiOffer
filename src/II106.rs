struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        fn dfs(graph: &Vec<Vec<i32>>, col: &mut Vec<i32>, x: usize, cur_col: i32) -> bool {
            col[x] = cur_col;
            for to in &graph[x] {
                let to = *to as usize;
                if col[to] == cur_col {
                    return false;
                }
                if col[to] == -1 && !dfs(graph, col, to, 1 - cur_col) {
                    return false;
                }
            }
            true
        }
        let n = graph.len();
        let mut col = vec![-1; n];
        for i in 0..n {
            if col[i] == -1 && !dfs(&graph, &mut col, i, 0) {
                return false;
            }
        }
        true
    }
}
