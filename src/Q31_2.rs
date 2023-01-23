struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        use std::collections::LinkedList;
        let mut i = 0;
        let len = pushed.len();
        let mut st = LinkedList::new();
        for val in popped {
            while i < len && (st.is_empty() || *st.back().unwrap() != val) {
                st.push_back(pushed[i]);
                i += 1;
            }
            if *st.back().unwrap() == val {
                st.pop_back();
            } else {
                return false;
            }
        }
        true
    }
}
