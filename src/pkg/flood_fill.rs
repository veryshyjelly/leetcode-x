#[allow(dead_code)]
pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let (m, n) = (image.len(), image[0].len());
    let mut visited = vec![false; m * n];
    let now_color = image[sr as usize][sc as usize];
    let mut stack = vec![to_index(sr as usize, sc as usize, n)];
    while let Some(pixel) = stack.pop() {
        let (row, col) = to_ro_col(pixel, n);
        visited[pixel] = true;
        image[row][col] = color;
        let mut adjacents = vec![];
        if row > 0 && image[row - 1][col] == now_color {
            adjacents.push(to_index(row - 1, col, n));
        }
        if row < m - 1 && image[row + 1][col] == now_color {
            adjacents.push(to_index(row + 1, col, n));
        }
        if col > 0 && image[row][col - 1] == now_color {
            adjacents.push(to_index(row, col - 1, n));
        }
        if col < n - 1 && image[row][col + 1] == now_color {
            adjacents.push(to_index(row, col + 1, n));
        }
        for a in adjacents {
            if !visited[a] {
                stack.push(a);
            }
        }
    }
    image
}

fn to_ro_col(index: usize, n: usize) -> (usize, usize) {
    (index / n, index % n)
}

fn to_index(i: usize, j: usize, n: usize) -> usize {
    i * n + j
}