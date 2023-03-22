struct Solution;

// 自己想的前后2遍遍历解法，可以优化为双指针
impl Solution {
    pub fn trap_mine(height: Vec<i32>) -> i32 {
        let mut cur = 0;
        let mut ret = 0;
        for &h in &height {
            if h > cur {
                cur = h;
            } else {
                ret += cur - h;
            }
        }
        let heightest = cur;
        cur = 0;
        for h in height.into_iter().rev() {
            if h == heightest {
                break;
            }
            if h > cur {
                cur = h;
            }
            ret -= heightest - cur;
        }
        ret
    }
}

// 双指针
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0usize, height.len() - 1);
        let (mut left_max, mut right_max) = (height[left], height[right]);
        let mut ret = 0;
        while left < right {
            if left_max < right_max {
                left += 1;
                if height[left] > left_max {
                    left_max = height[left];
                } else {
                    ret += left_max - height[left];
                }
            } else {
                right -= 1;
                if height[right] > right_max {
                    right_max = height[right];
                } else {
                    ret += right_max - height[right];
                }
            }
        }
        ret
    }
}
