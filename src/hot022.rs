struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn dfs(left_res: i32, right_res: i32, cur: &mut String, res: &mut Vec<String>) {
            if left_res == 0 && right_res == 0 {
                res.push(cur.clone());
                return;
            }
            if left_res > 0 {
                cur.push('(');
                dfs(left_res - 1, right_res, cur, res);
                cur.pop();
            }
            if right_res > left_res {
                cur.push(')');
                dfs(left_res, right_res - 1, cur, res);
                cur.pop();
            }
        }
        let mut res = vec![];
        dfs(n, n, &mut String::new(), &mut res);
        res
    }
}
