#[allow(dead_code)]
pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    for i in 1..n {
        grid[0][i] += grid[0][i - 1];
    }
    for i in 1..m {
        grid[i][0] += grid[i - 1][0];
    }

    for i in 1..m {
        for j in 1..n {
            grid[i][j] += i32::min(grid[i - 1][j], grid[i][j - 1]);
        }
    }

    grid[m - 1][n - 1]
}