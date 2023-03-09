struct Solution;

// 寻找第k大数
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        fn find_kth_num(nums1: &Vec<i32>, nums2: &Vec<i32>, mut k: usize) -> i32 {
            let (m, n) = (nums1.len(), nums2.len());
            let (mut index1, mut index2) = (0, 0);
            loop {
                if index1 == m {
                    return nums2[index2 + k - 1];
                } else if index2 == n {
                    return nums1[index1 + k - 1];
                } else if k == 1 {
                    return nums1[index1].min(nums2[index2]);
                } else {
                    let (new_index1, new_index2) = (
                        (m - 1).min(index1 + (k - 2) / 2),
                        (n - 1).min(index2 + (k - 2) / 2),
                    );
                    if nums1[new_index1] < nums2[new_index2] {
                        k -= new_index1 - index1 + 1;
                        index1 = new_index1 + 1;
                    } else {
                        k -= new_index2 - index2 + 1;
                        index2 = new_index2 + 1;
                    }
                }
            }
        }
        let tot = nums1.len() + nums2.len();
        if tot & 1 == 1 {
            find_kth_num(&nums1, &nums2, (tot + 1) / 2) as f64
        } else {
            (find_kth_num(&nums1, &nums2, tot / 2) + find_kth_num(&nums1, &nums2, tot / 2 + 1))
                as f64
                / 2.0
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
            (Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]))
        );
        assert_eq!(
            3.5,
            (Solution::find_median_sorted_arrays(vec![3, 4], vec![]))
        );
        assert_eq!(
            100000.5,
            (Solution::find_median_sorted_arrays(vec![100001], vec![100000]))
        );
        assert_eq!(
            1.5,
            (Solution::find_median_sorted_arrays(vec![1, 2], vec![-1, 3]))
        );
        assert_eq!(
            2.5,
            (Solution::find_median_sorted_arrays(vec![3], vec![1, 2, 4]))
        );
        assert_eq!(
            3.0,
            (Solution::find_median_sorted_arrays(vec![4], vec![1, 2, 3, 5]))
        );
        assert_eq!(
            4.0,
            dbg!(Solution::find_median_sorted_arrays(
                vec![5, 6],
                vec![1, 2, 3, 4, 7]
            ))
        );
    }
}
