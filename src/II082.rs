struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn get_res(
            candidates: &Vec<i32>,
            target: i32,
            i_start: usize,
            vs: &mut Vec<bool>,
            cur: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            let n = candidates.len();
            // 剪枝
            if i_start < n && candidates[n - 1] * ((n - i_start) as i32) < target {
                return;
            }
            if target < 0 {
                return;
            } else if target == 0 {
                res.push(cur.clone());
                return;
            }
            for i in i_start..candidates.len() {
                if i > 0 && !vs[i - 1] && candidates[i] == candidates[i - 1] {
                    continue;
                }
                cur.push(candidates[i]);
                vs[i] = true;
                get_res(candidates, target - candidates[i], i + 1, vs, cur, res);
                vs[i] = false;
                cur.pop();
            }
        }
        candidates.sort();
        let mut res = vec![];
        get_res(
            &candidates,
            target,
            0,
            &mut vec![false; candidates.len()],
            &mut vec![],
            &mut res,
        );
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II082() {
        assert_eq!(
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
            dbg!(Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8))
        );
    }
}
