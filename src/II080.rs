struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn get_res(n: i32, k: i32, i_start: i32, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if n - i_start < k {
                // 剪枝
                return;
            }
            if k == 0 {
                res.push(cur.clone());
                return;
            }
            for i in i_start + 1..=n {
                cur.push(i);
                get_res(n, k - 1, i, cur, res);
                cur.pop();
            }
        }
        let mut res = vec![];
        get_res(n, k, 0, &mut vec![], &mut res);
        res
    }
}
