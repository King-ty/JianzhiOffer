struct Solution;

impl Solution {
    pub fn verify_postorder(postorder: Vec<i32>) -> bool {
        let mut root = i32::MAX;
        let mut stack = Vec::new();
        for val in postorder.into_iter().rev() {
            if val > root {
                return false;
            }
            while !stack.is_empty() && stack[stack.len() - 1] > val {
                root = stack.pop().unwrap();
            }
            stack.push(val);
        }
        true
    }
}
