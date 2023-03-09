struct Solution;

/// 我自己思路的一些尝试，全都是不正确的QwQ
/// 思路类似于划分数组，如果nums1[mid1] < nums2[mid2]，则nums1的前半段在前半段，nums2的后半段在后半段
/// 然后对剩下未确定的部分继续查找
/// 这个方法对于2的幂次理论可行，但是对于其他情况需要讨论边界，我最后没有弄明白QwQ
impl Solution {
    pub fn find_median_sorted_arrays_0(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        use std::cmp;
        let (m, n) = (nums1.len(), nums2.len());
        let half_tot = (m + n) / 2 + 1;
        let (mut l1, mut l2, mut r1, mut r2) = (0, 0, m, n);
        while l1 < r1 && l2 < r2 {
            let (mid1, mid2) = (l1 + (r1 - l1) / 2, l2 + (r2 - l2) / 2);
            if nums1[mid1] < nums2[mid2] {
                l1 = mid1 + 1;
                r2 = mid2;
            } else if nums1[mid1] > nums2[mid2] {
                r1 = mid1;
                l2 = mid2 + 1;
            } else {
                l1 = mid1 + 1;
                l2 = mid2 + 1;
                break;
            }
        }
        // println!("{l1},{l2}");
        if l1 + l2 > half_tot {
            l2 -= 1;
        } else if l1 + l2 < half_tot {
            if l2 == n {
                l1 = half_tot - l2;
            } else if l1 == m {
                l2 = half_tot - l1;
            } else {
                if nums1[l1] < nums2[l2] {
                    l1 = half_tot - l2;
                } else {
                    l2 = half_tot - l1;
                }
            }
        }

        println!("{l1},{l2}");

        if (m + n) & 1 == 1 {
            (if l1 == 0 {
                nums2[l2 - 1]
            } else if l2 == 0 {
                nums1[l1 - 1]
            } else {
                cmp::max(nums1[l1 - 1], nums2[l2 - 1])
            }) as f64
        } else {
            if l1 == 0 {
                (nums2[l2 - 1] + nums2[l2 - 2]) as f64 / 2.0
            } else if l2 == 0 {
                (nums1[l1 - 1] + nums1[l1 - 2]) as f64 / 2.0
            } else if l1 == 1 && l2 == 1 {
                (nums1[l1 - 1] + nums2[l2 - 1]) as f64 / 2.0
            } else {
                if nums1[l1 - 1] > nums2[l2 - 1] {
                    if l1 > 1 {
                        (nums1[l1 - 1] + cmp::max(nums1[l1 - 2], nums2[l2 - 1])) as f64 / 2.0
                    } else {
                        (nums1[l1 - 1] + nums2[l2 - 1]) as f64 / 2.0
                    }
                } else {
                    if l2 > 1 {
                        (nums2[l2 - 1] + cmp::max(nums1[l1 - 1], nums2[l2 - 2])) as f64 / 2.0
                    } else {
                        (nums1[l1 - 1] + nums2[l2 - 1]) as f64 / 2.0
                    }
                }
            }
        }
    }
}

impl Solution {
    pub fn find_median_sorted_arrays_1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        use std::cmp;
        let (m, n) = (nums1.len(), nums2.len());
        let half_tot = (m + n) / 2;
        let (mut l1, mut l2, mut r1, mut r2) = (0, 0, m, n);
        while l1 + l2 < half_tot {
            let (mid1, mid2) = (l1 + (r1 - l1 - 1) / 2, l2 + (r2 - l2 - 1) / 2);
            if nums1[mid1] < nums2[mid2] {
                l1 = l1 + (r1 - l1) / 2;
                r2 = l2 + (r2 - l2 + 1) / 2;
            } else if nums1[mid1] > nums2[mid2] {
                l2 = l2 + (r2 - l2) / 2;
                r1 = l1 + (r1 - l1 + 1) / 2;
            } else {
                l1 = l1 + (r1 - l1) / 2;
                l2 = l2 + (r2 - l2) / 2;
                break;
            }
        }

        println!("{l1},{l2}");

        if (m + n) & 1 == 1 {
            (if l1 == m {
                nums2[l2]
            } else if l2 == n {
                nums1[l1]
            } else {
                cmp::min(nums1[l1], nums2[l2])
            }) as f64
        } else {
            (nums1[l1] + nums2[l2]) as f64 / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hot004() {
        assert_eq!(
            2.5,
            (Solution::find_median_sorted_arrays_0(vec![1, 2], vec![3, 4]))
        );
        assert_eq!(
            3.5,
            (Solution::find_median_sorted_arrays_0(vec![3, 4], vec![]))
        );
        assert_eq!(
            100000.5,
            (Solution::find_median_sorted_arrays_0(vec![100001], vec![100000]))
        );
        assert_eq!(
            1.5,
            (Solution::find_median_sorted_arrays_0(vec![1, 2], vec![-1, 3]))
        );
        assert_eq!(
            2.5,
            (Solution::find_median_sorted_arrays_0(vec![3], vec![1, 2, 4]))
        );
        assert_eq!(
            3.0,
            (Solution::find_median_sorted_arrays_0(vec![4], vec![1, 2, 3, 5]))
        );
        assert_eq!(
            4.0,
            dbg!(Solution::find_median_sorted_arrays_0(
                vec![5, 6],
                vec![1, 2, 3, 4, 7]
            ))
        );
    }
}
