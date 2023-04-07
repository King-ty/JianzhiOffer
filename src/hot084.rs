struct Solution;

// 使用单调递增栈，弹出时更新弹出的点，
// 其左界为栈内上一个元素的下一个位置，右界为新加入元素的上一个位置。
// 初始在单调栈中放入-1来处理边界
// 在heights末尾放入0保证所有元素都会被弹出
// 这样实现比较巧妙简洁，第二次做我依然没想到这种思路，还是看了眼之前的代码才明白QwQ
impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.push(0);
        let mut st = vec![-1];
        let mut ret = 0;
        for i in 0..heights.len() {
            while st.len() > 1 && heights[*st.last().unwrap() as usize] > heights[i] {
                let height = heights[st.pop().unwrap() as usize];
                let width = i as i32 - *st.last().unwrap() - 1;
                ret = ret.max(height * width);
            }
            st.push(i as i32);
        }
        ret
    }
}
