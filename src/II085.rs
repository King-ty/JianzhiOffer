struct Solution;

// 来自题解：可以初始化l_num和r_num为n，递归用-1，以此优化掉n参数
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn get_res(n: i32, l_num: i32, r_num: i32, cur: &mut String, res: &mut Vec<String>) {
            if l_num + r_num == n * 2 {
                res.push(cur.clone());
                return;
            }
            if l_num < n {
                cur.push('(');
                get_res(n, l_num + 1, r_num, cur, res);
                cur.pop();
            }
            if r_num < l_num {
                cur.push(')');
                get_res(n, l_num, r_num + 1, cur, res);
                cur.pop();
            }
        }
        let mut res = vec![];
        get_res(n, 0, 0, &mut String::new(), &mut res);
        res
    }
}
