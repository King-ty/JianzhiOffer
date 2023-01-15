pub struct Solution;

impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        if m == 0 {
            return false;
        }
        let n = matrix[0].len();
        if n == 0 {
            return false;
        }
        let mut i = 0;
        let mut j = n - 1;
        while i < m {
            if matrix[i][j] == target {
                return true;
            } else if matrix[i][j] > target {
                if j == 0 {
                    break;
                }
                j -= 1;
            } else if matrix[i][j] < target {
                i += 1;
            }
        }
        false
    }
}

impl Solution {
    fn find_y(matrix: &Vec<Vec<i32>>, target: i32) -> usize {
        let mut l = 0;
        let mut r = matrix.len();
        let mut mid;
        while l < r {
            mid = l + (r - l) / 2;
            if matrix[mid][0] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }

    fn find_x(line: &Vec<i32>, target: i32) -> usize {
        let mut l = 0;
        let mut r = line.len();
        let mut mid;
        while l < r {
            mid = l + (r - l) / 2;
            if line[mid] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }

    pub fn find_number_in2_d_array_old(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let y = Self::find_y(&matrix, target);
        let x = Self::find_x(&matrix[y], target);
        target == matrix[y][x]
    }
}
