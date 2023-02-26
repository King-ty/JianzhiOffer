struct Solution;

// partition + random
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        use rand::prelude::random;
        fn partition(mut l: usize, mut r: usize, nums: &mut Vec<i32>) -> usize {
            let rd = random::<usize>() % (r - l + 1);
            nums.swap(l, l + rd);
            let cur = nums[l];
            while l < r {
                while l < r && nums[r] <= cur {
                    r -= 1;
                }
                nums[l] = nums[r];
                while l < r && nums[l] >= cur {
                    l += 1;
                }
                nums[r] = nums[l];
            }
            nums[l] = cur;
            l
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut pos = partition(l, r, &mut nums);
        let kk = (k - 1) as usize;
        while pos != kk {
            if pos < kk {
                l = pos + 1;
            } else {
                r = pos - 1;
            }
            pos = partition(l, r, &mut nums);
        }
        nums[kk]
    }
}
