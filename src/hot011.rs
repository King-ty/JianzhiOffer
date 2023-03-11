struct Solution;

// 单调栈+二分，不是很好的做法
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        fn half_search(arr: &Vec<(i32, usize)>, x: i32) -> usize {
            let (mut l, mut r) = (0, arr.len());
            while l < r {
                let mid = (l + r) / 2;
                if arr[mid].0 > x {
                    r = mid;
                } else if arr[mid].0 < x {
                    l = mid + 1;
                } else {
                    return arr[mid].1;
                }
            }
            arr[l].1
        }
        let mut mono_st: Vec<(i32, usize)> = vec![];
        let mut res = 0;
        for (i, &h) in height.iter().enumerate() {
            if mono_st.len() == 0 || mono_st[mono_st.len() - 1].0 < h {
                mono_st.push((h, i));
            } else {
                res = res.max(h * ((i - half_search(&mono_st, h)) as i32));
            }
        }
        mono_st.clear();
        for (i, h) in height.into_iter().rev().enumerate() {
            if mono_st.len() == 0 || mono_st[mono_st.len() - 1].0 < h {
                mono_st.push((h, i));
            } else {
                res = res.max(h * ((i - half_search(&mono_st, h)) as i32));
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hot011() {
        assert_eq!(1, dbg!(Solution::max_area(vec![1, 2])));
    }
}
