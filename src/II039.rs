struct Solution;

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        use std::cmp::max;
        heights.push(0);
        let n = heights.len();
        let mut ret = 0;
        let mut st = vec![-1];
        for i in 0..n {
            while st.len() > 1 && heights[st[st.len() - 1] as usize] > heights[i] {
                let height = heights[st.pop().unwrap() as usize];
                let width = i as i32 - st[st.len() - 1] - 1;
                ret = max(ret, height * width);
            }
            st.push(i as i32);
        }
        ret
    }
}
