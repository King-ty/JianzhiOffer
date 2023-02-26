struct Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut ord = [1009; 1009];
        for (index, num) in arr2.into_iter().enumerate() {
            ord[num as usize] = index;
        }
        arr1.sort_by(|a, b| match ord[*a as usize].cmp(&ord[*b as usize]) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => (*a).cmp(b),
        });

        arr1
    }
}
