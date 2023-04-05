struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, index: usize) {
            if index == nums.len() {
                res.push(cur.clone());
                return;
            }

            dfs(nums, res, cur, index + 1);
            cur.push(nums[index]);
            dfs(nums, res, cur, index + 1);
            cur.pop();
        }
        let mut res = vec![];
        dfs(&nums, &mut res, &mut vec![], 0);
        res
    }
}
