struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        ((0.5 * (x as f64).ln()).exp() + 1e-6) as i32
    }
}
