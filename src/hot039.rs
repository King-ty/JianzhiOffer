struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(
            candidates: &mut Vec<i32>,
            target: i32,
            i_start: usize,
            cur: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if target == 0 {
                res.push(cur.clone());
                return;
            }
            for i in i_start..candidates.len() {
                if candidates[i] > target {
                    break;
                }
                cur.push(candidates[i]);
                dfs(candidates, target - candidates[i], i, cur, res);
                cur.pop();
            }
        }
        candidates.sort();
        let mut ret = vec![];
        dfs(&mut candidates, target, 0, &mut vec![], &mut ret);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hot039() {
        dbg!(Solution::combination_sum(vec![2, 3, 6, 7], 7));
    }
}
