#[allow(dead_code)]
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for level in 0..n / 2 {
        let mut temp = vec![0; n - 2 * level - 1];
        for i in 0..n - 2 * level - 1 {
            temp[i] = matrix[level][i + level];
            matrix[level][i + level] = matrix[n - (i + level) - 1][level];
            matrix[n - (i + level) - 1][level] = matrix[n - level - 1][n - (i + level) - 1];
            matrix[n - level - 1][n - (i + level) - 1] = matrix[i + level][n - level - 1];
            matrix[i + level][n - level - 1] = temp[i];
        }
    }
}