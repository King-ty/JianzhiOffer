struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (mut a, mut b) = (a.bytes().rev(), b.bytes().rev());
        let mut ret = vec![];
        let mut carry = 0;
        loop {
            match (a.next(), b.next()) {
                (Some(mut x), Some(y)) => {
                    x -= '0' as u8;
                    x += y - '0' as u8 + carry;
                    ret.push((x % 2 + '0' as u8) as char);
                    carry = x / 2;
                }
                (Some(mut x), None) | (None, Some(mut x)) => {
                    x = x - '0' as u8 + carry;
                    ret.push((x % 2 + '0' as u8) as char);
                    carry = x / 2;
                }
                (None, None) => {
                    if carry == 1 {
                        ret.push('1');
                    }
                    break;
                }
            }
        }
        ret.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II002() {
        assert_eq!(
            "101",
            dbg!(Solution::add_binary("11".to_string(), "10".to_string()))
        );
    }
}
