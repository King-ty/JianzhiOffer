struct Solution;

impl Solution {
    pub fn divide(a: i32, b: i32) -> i32 {
        if a == i32::MIN {
            if b == -1 {
                return i32::MAX;
            } else if b == 1 {
                return i32::MIN;
            }
        }
        if b == i32::MIN {
            return if a == i32::MIN { 1 } else { 0 };
        }
        if a == 0 {
            return 0;
        }
        let mut rev = false;
        let (mut a, mut b) = (a, b);
        if a > 0 {
            a = -a;
            rev = !rev;
        }
        if b > 0 {
            b = -b;
            rev = !rev;
        }
        let mut temp = b;
        let mut nums = vec![b];
        while temp >= a - temp {
            temp += temp;
            nums.push(temp);
        }
        let mut ret = 0;
        for i in (0..nums.len()).rev() {
            if nums[i] >= a {
                a -= nums[i];
                ret |= 1 << i;
            }
        }
        if rev {
            -ret
        } else {
            ret
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_II001() {
        use super::Solution;
        assert_eq!(0, dbg!(Solution::divide(-2147483648, 2)));
    }
}
