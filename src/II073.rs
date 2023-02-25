struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        use std::cmp::max;
        let mut tot: i64 = 0;
        let mut ma = 0;
        for pile in &piles {
            tot += *pile as i64;
            ma = max(ma, *pile);
        }
        let h_64 = h as i64;
        // let n = piles.len() as i64;
        let (mut l, mut r) = (((tot + h_64 - 1) / h_64) as usize, ma as usize);
        let check = |x| {
            let x = x as i32;
            let mut hh = 0;
            for pile in &piles {
                hh += (*pile + x - 1) / x;
            }
            return hh <= h;
        };
        while l < r {
            let mid = l + (r - l) / 2;
            if check(mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l as i32
    }
}
