#[allow(dead_code)]
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut lo, mut hi) = (0, m * n - 1);
    let max_hi = m * n;

    while lo <= hi && hi < max_hi {
        let mid = (lo + hi) / 2;
        let value = matrix[mid / n][mid % n];
        if value < target {
            lo = mid + 1;
        } else if value > target {
            hi = mid - 1;
        } else {
            return true;
        }
    }

    false
}