struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn get_res(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, start_i: usize) {
            res.push(cur.clone());
            for i in start_i..nums.len() {
                cur.push(nums[i]);
                get_res(nums, res, cur, i + 1);
                cur.pop();
            }
        }
        let mut res = vec![];
        get_res(&nums, &mut res, &mut vec![], 0);
        res
    }
}
