#[allow(dead_code)]
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (m, n) = (matrix.len(), matrix[0].len());

    let mut colus = vec![false; n];
    for row in matrix.iter_mut() {
        let mut x = false;
        for (j, m) in row.iter_mut().enumerate() {
            if m == &0 {
                colus[j] = true;
                x = true;
            }
        }
        if x {
            *row = vec![0; n];
        }
    }

    for c in 0..n {
        if !colus[c] {
            continue;
        }
        for i in 0..m {
            matrix[i][c] = 0;
        }
    }
}