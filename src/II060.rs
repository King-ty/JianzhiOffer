struct Solution;

// 因为堆专题，所以用堆做的。也可以把元组用Partition找出前k大
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashMap};
        let mut heap = BinaryHeap::new();
        let mut mp = HashMap::new();
        for num in &nums {
            *mp.entry(*num).or_default() += 1;
        }
        for num in nums {
            let fre: i32 = *mp.get(&num).unwrap();
            if fre <= 0 {
                continue;
            }
            *mp.entry(num).or_default() = 0;
            if k as usize > heap.len() {
                heap.push(Reverse((fre, num)));
            } else if heap.peek().unwrap().0 .0 < fre {
                heap.pop();
                heap.push(Reverse((fre, num)));
            }
        }
        let mut ret = Vec::with_capacity(heap.len());
        while let Some(rev) = heap.pop() {
            ret.push(rev.0 .1);
        }
        ret
    }
}
