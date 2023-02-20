struct Solution;

use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};
#[derive(PartialEq, Eq, Ord)]
struct Pair(usize, usize, i32);
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.2.cmp(&other.2))
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (nums1.len(), nums2.len());
        let mut ret = vec![];
        let mut shp = BinaryHeap::new();
        for i in 0..m {
            shp.push(Reverse(Pair(i, 0, nums1[i] + nums2[0])));
        }
        let mut k = k;
        while k > 0 && !shp.is_empty() {
            k -= 1;
            let pair = shp.pop().unwrap().0;
            ret.push(vec![nums1[pair.0], nums2[pair.1]]);
            if pair.1 < n - 1 {
                shp.push(Reverse(Pair(
                    pair.0,
                    pair.1 + 1,
                    nums1[pair.0] + nums2[pair.1 + 1],
                )));
            }
        }
        ret
    }
}

// impl Solution {
//     pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
//         let (m, n) = (nums1.len(), nums2.len());
//         let mut ret = vec![];
//         let mut shp = BinaryHeap::new();
//         for i in 0..m {
//             shp.push(Reverse(Pair(i, 0, nums1[i] + nums2[0])));
//         }
//         for _ in 0..k {
//             if let Some(rev) = shp.pop() {
//                 let pair = rev.0;
//                 ret.push(vec![nums1[pair.0], nums2[pair.1]]);
//                 if pair.1 < n - 1 {
//                     shp.push(Reverse(Pair(
//                         pair.0,
//                         pair.1 + 1,
//                         nums1[pair.0] + nums2[pair.1 + 1],
//                     )));
//                 }
//             } else {
//                 break;
//             }
//         }
//         ret
//     }
// }
