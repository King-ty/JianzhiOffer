struct Solution;

impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        fn partition(arr: &mut Vec<i32>, mut l: usize, mut r: usize, k: usize) {
            let (ll, rr) = (l, r);
            let cur = arr[l];
            while l < r {
                while l < r && arr[r] >= cur {
                    r -= 1;
                }
                arr[l] = arr[r];
                while l < r && arr[l] <= cur {
                    l += 1;
                }
                arr[r] = arr[l];
            }
            arr[l] = cur;
            if l == k || l == k - 1 {
                return;
            } else if l < k - 1 {
                partition(arr, l + 1, rr, k);
            } else {
                partition(arr, ll, l - 1, k);
            }
        }
        let mut arr = arr;
        let len = arr.len();
        partition(&mut arr, 0, len - 1, k as usize);
        arr.resize(k as usize, 0);
        arr
    }
}
