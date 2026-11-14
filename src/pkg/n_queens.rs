#[allow(dead_code)]
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut res = vec![];
    let mut temp = vec![];
    let mut col = vec![false; n];
    let mut diag1 = vec![false; n * 2 - 1];
    let mut diag2 = vec![false; n * 2 - 1];
    queens_backtrack(0, n, &mut res, &mut temp, &mut col, &mut diag1, &mut diag2);
    res
}

pub fn queens_backtrack(
    y: usize,
    n: usize,
    res: &mut Vec<Vec<String>>,
    temp: &mut Vec<String>,
    col: &mut Vec<bool>,
    diag1: &mut Vec<bool>,
    diag2: &mut Vec<bool>,
) {
    if y == n {
        res.push(temp.clone());
        return;
    }

    for x in 0..n {
        if col[x] || diag1[x + y] || diag2[x - y + n - 1] {
            continue;
        }
        col[x] = true;
        diag1[x + y] = true;
        diag2[x - y + n - 1] = true;
        temp.push(
            vec![
                vec!["."; x].join("").as_str(),
                "Q",
                vec!["."; n - x - 1].join("").as_str(),
            ]
            .join(""),
        );
        queens_backtrack(y + 1, n, res, temp, col, diag1, diag2);
        temp.pop();
        col[x] = false;
        diag1[x + y] = false;
        diag2[x - y + n - 1] = false;
    }
}