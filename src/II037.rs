struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        for asteroid in asteroids {
            if asteroid > 0 {
                ret.push(asteroid);
            } else {
                loop {
                    if ret.is_empty() {
                        ret.push(asteroid);
                        break;
                    }
                    let temp = ret[ret.len() - 1];
                    if temp < 0 {
                        ret.push(asteroid);
                        break;
                    } else if temp > asteroid.abs() {
                        break;
                    } else {
                        ret.pop();
                        if temp == asteroid.abs() {
                            break;
                        }
                    }
                }
            }
        }
        ret
    }
}
