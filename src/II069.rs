struct Solution;

// 朴素三分查找法，由于本题一定是凸而不是凹，所以二分即可
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, arr.len() - 1);
        while r - l > 2 {
            let (ml, mr) = (l + (r - l) / 3, r - (r - l) / 3);
            if arr[ml] < arr[mr] {
                l = ml;
            } else {
                r = mr;
            }
        }
        (l + 1) as i32
    }
}
