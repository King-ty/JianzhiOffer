struct Solution;

impl Solution {
    pub fn str_to_int(str: String) -> i32 {
        if str.is_empty() {
            return 0;
        }
        let chars: Vec<u8> = str.bytes().collect();
        let mut i = 0;
        let len = str.len();
        let mut w = 1;
        let mut ret = 0i64;
        while i < len && (chars[i] == ' ' as u8) {
            i += 1;
        }
        if i >= len {
            return 0;
        }
        if chars[i] == '+' as u8 {
            i += 1;
        } else if chars[i] == '-' as u8 {
            w = -1;
            i += 1;
        }
        while i < len && (chars[i] >= '0' as u8 && chars[i] <= '9' as u8) {
            ret = ret * 10 + (chars[i] - '0' as u8) as i64;
            if ret > i32::MAX as i64 {
                break;
            }
            i += 1;
        }
        ret *= w as i64;
        if ret > i32::MAX as i64 {
            i32::MAX
        } else if ret < i32::MIN as i64 {
            i32::MIN
        } else {
            ret as i32
        }
    }
}
