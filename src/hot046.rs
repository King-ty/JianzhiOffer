struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn get_ans(nums: &mut Vec<i32>, i_start: usize, ret: &mut Vec<Vec<i32>>) {
            let n = nums.len();
            if i_start == n {
                ret.push(nums.clone());
                return;
            }
            for i in i_start..n {
                nums.swap(i_start, i);
                get_ans(nums, i_start + 1, ret);
                nums.swap(i_start, i);
            }
        }
        let mut ret = vec![];
        get_ans(&mut nums, 0, &mut ret);
        ret
    }
}
