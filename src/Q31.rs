// 不知道为啥空间占用超过0%
struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut i = 0;
        let len = pushed.len();
        let mut st = Vec::new();
        for val in popped {
            while i < len && (st.is_empty() || st[st.len() - 1] != val) {
                st.push(pushed[i]);
                i += 1;
            }
            if st[st.len() - 1] == val {
                st.pop();
            } else {
                return false;
            }
        }
        true
    }
}
