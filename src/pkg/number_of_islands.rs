#[allow(dead_code)]
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut visited = vec![false; m * n];
    let mut answer = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '0' {
                visited[to_index(i, j, n)] = true;
            }
        }
    }

    let mut index = 0;
    loop {
        while index < visited.len() && visited[index] {
            index += 1;
        }
        if !visited[index] {
            visited[index] = true;
            answer += 1;
            let mut stack = vec![index];
            while let Some(cell) = stack.pop() {
                let (row, col) = to_ro_col(cell, n);
                let mut adjacents = vec![];
                if row > 0 {
                    adjacents.push(to_index(row - 1, col, n));
                }
                if row < m - 1 {
                    adjacents.push(to_index(row + 1, col, n));
                }
                if col > 0 {
                    adjacents.push(to_index(row, col - 1, n));
                }
                if col < n - 1 {
                    adjacents.push(to_index(row, col + 1, n));
                }
                for a in adjacents {
                    if !visited[a] {
                        stack.push(a);
                    }
                }
            }
        } else {
            break;
        }
    }

    answer
}

fn to_ro_col(index: usize, n: usize) -> (usize, usize) {
    (index / n, index % n)
}

fn to_index(i: usize, j: usize, n: usize) -> usize {
    i * n + j
}