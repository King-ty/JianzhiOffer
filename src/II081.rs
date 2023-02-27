struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn get_res(
            candidates: &Vec<i32>,
            target: i32,
            i_start: usize,
            cur: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            // 其实可以加一个后缀和剪枝
            if target < 0 {
                return;
            } else if target == 0 {
                res.push(cur.clone());
                return;
            }
            for i in i_start..candidates.len() {
                cur.push(candidates[i]);
                get_res(candidates, target - candidates[i], i, cur, res);
                cur.pop();
            }
        }
        let mut res = vec![];
        get_res(&candidates, target, 0, &mut vec![], &mut res);
        res
    }
}
