struct Solution;

impl Solution {
    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        let mut ss = 1;
        let len = a.len();
        let mut pre = Vec::with_capacity(len);
        let mut bac = Vec::with_capacity(len);
        let mut ret = Vec::with_capacity(len);
        pre.push(1);
        for num in &a {
            ss *= *num;
            pre.push(ss);
        }
        ss = 1;
        bac.push(1);
        for num in a.iter().rev() {
            ss *= *num;
            bac.push(ss);
        }
        for i in 0..len {
            ret.push(pre[i] * bac[len - i - 1]);
        }
        ret
    }
}
