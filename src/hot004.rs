struct Solution;

// 划分数组
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        use std::cmp;
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
        let (m, n) = (nums1.len(), nums2.len());
        let (mut l, mut r) = (0, m);
        let (mut midl, mut midr) = (0, 0);
        while l <= r {
            let i = (l + r) / 2;
            let j = (m + n + 1) / 2 - i;
            let numi = if i == m { i32::MAX } else { nums1[i] };
            let numj = if j == n { i32::MAX } else { nums2[j] };
            let numim1 = if i == 0 { i32::MIN } else { nums1[i - 1] };
            let numjm1 = if j == 0 { i32::MIN } else { nums2[j - 1] };
            if numim1 <= numj {
                midl = cmp::max(numim1, numjm1);
                midr = cmp::min(numi, numj);
                l = i + 1;
            } else {
                r = i - 1;
            }
        }
        if (m + n) & 1 == 1 {
            midl as f64
        } else {
            (midl + midr) as f64 / 2.0
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
