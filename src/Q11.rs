pub struct Solution;

impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        use std::cmp::min;
        let mut l = 0;
        let mut r = numbers.len();
        let len = r;
        let mut mid;
        let mut ret = numbers[l];
        while l < r {
            mid = l + (r - l) / 2;
            if numbers[mid] > numbers[l] {
                l = mid + 1;
                if l < len {
                    ret = min(ret, numbers[l]);
                }
            } else if numbers[mid] < numbers[l] {
                r = mid;
            } else {
                l += 1;
                if l < len {
                    ret = min(ret, numbers[l]);
                }
            }
        }
        ret
    }
}
