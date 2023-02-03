// 担心爆i32使用了i64，结果实际上没有爆，还拖累了速度和内存

struct NumMatrix {
    matrix_pre_sum: Vec<Vec<i64>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut matrix_pre_sum = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                matrix_pre_sum[i + 1][j + 1] = matrix_pre_sum[i][j + 1] + matrix_pre_sum[i + 1][j]
                    - matrix_pre_sum[i][j]
                    + matrix[i][j] as i64;
            }
        }
        Self { matrix_pre_sum }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        (self.matrix_pre_sum[row2 + 1][col2 + 1]
            - self.matrix_pre_sum[row1][col2 + 1]
            - self.matrix_pre_sum[row2 + 1][col1]
            + self.matrix_pre_sum[row1][col1]) as i32
    }
}
