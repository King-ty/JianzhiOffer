struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st = vec![];
        for c in s.into_bytes() {
            match c {
                b'(' | b'[' | b'{' => st.push(c),
                b')' => match st.pop() {
                    Some(left) => {
                        if left != b'(' {
                            return false;
                        }
                    }
                    None => return false,
                },
                b']' => match st.pop() {
                    Some(left) => {
                        if left != b'[' {
                            return false;
                        }
                    }
                    None => return false,
                },
                b'}' => match st.pop() {
                    Some(left) => {
                        if left != b'{' {
                            return false;
                        }
                    }
                    None => return false,
                },
                _ => (),
            }
        }
        st.is_empty()
    }
}
