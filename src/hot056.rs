struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let (mut cur_l, mut cur_r) = (intervals[0][0], intervals[0][1]);
        let mut ret = vec![];
        for interval in intervals.into_iter().skip(1) {
            if interval[0] > cur_r {
                ret.push(vec![cur_l, cur_r]);
                cur_l = interval[0];
                cur_r = interval[1];
            } else if interval[1] > cur_r {
                cur_r = interval[1];
            }
        }
        ret.push(vec![cur_l, cur_r]);
        ret
    }
}
