struct Solution;

// 题解解法，栈里一定有上一个未匹配的右括号的位置，一开始栈里-1
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ret = 0;
        let mut st = vec![-1];
        for (i, c) in s.bytes().enumerate() {
            if c == b'(' {
                st.push(i as i32);
            } else {
                st.pop();
                if st.is_empty() {
                    st.push(i as i32)
                } else {
                    ret = ret.max(i as i32 - *st.last().unwrap());
                }
            }
        }
        ret
    }
}

// 我自己的解法，需要在栈里额外保留上一次匹配的长度
impl Solution {
    pub fn longest_valid_parentheses_mine(s: String) -> i32 {
        let mut ret = 0;
        let mut st = vec![];
        let bytes = s.as_bytes();
        let mut last_len = 0;
        for i in 0..s.len() {
            if bytes[i] == b'(' {
                if i > 0 && bytes[i - 1] == b')' {
                    st.push((i, last_len));
                } else {
                    st.push((i, 0));
                }
            } else {
                if let Some((left, pre_len)) = st.pop() {
                    let len = i + 1 - left + pre_len;
                    last_len = len;
                    ret = ret.max(len);
                } else {
                    last_len = 0;
                }
            }
        }
        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hot032() {
        assert_eq!(0, Solution::longest_valid_parentheses("))".to_string()));
        assert_eq!(4, Solution::longest_valid_parentheses(")()())".to_string()));
        assert_eq!(0, Solution::longest_valid_parentheses("".to_string()));
        assert_eq!(4, Solution::longest_valid_parentheses("(())))".to_string()));
        assert_eq!(2, Solution::longest_valid_parentheses("()(()".to_string()));
        assert_eq!(6, Solution::longest_valid_parentheses("()(())".to_string()));
        assert_eq!(
            4,
            Solution::longest_valid_parentheses(")()())()()(".to_string())
        );
        assert_eq!(
            10,
            Solution::longest_valid_parentheses("(((()(())))".to_string())
        );
        assert_eq!(
            10,
            Solution::longest_valid_parentheses("((((((())))()".to_string())
        );
    }
}
