struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut l, mut r) = (0, nums.len());
        let mut i = l;
        while i < r {
            if nums[i] == 0 {
                nums.swap(l, i);
                l += 1;
                if i < l {
                    i = l;
                }
            } else if nums[i] == 2 {
                r -= 1;
                nums.swap(r, i);
            } else {
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hot075() {
        dbg!(Solution::sort_colors(&mut vec![0, 2, 1]));
    }
}
