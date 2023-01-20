struct Solution;

impl Solution {
    pub fn is_straight(mut nums: Vec<i32>) -> bool {
        nums.sort();
        let mut king = 0;
        let mut pre = 0;
        for num in nums {
            if num == 0 {
                king += 1;
                continue;
            }
            if pre > 0 {
                if num == pre {
                    return false;
                }
                if num - pre > 1 {
                    king -= num - pre - 1;
                    if king < 0 {
                        return false;
                    }
                }
            }
            pre = num;
        }
        true
    }
}
