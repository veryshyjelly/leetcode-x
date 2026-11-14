#[allow(dead_code)]
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 1 {
        return vec![vec![1]];
    }

    let mut answer = generate(num_rows - 1);
    let mut new_row = vec![1];

    for i in 1..num_rows as usize - 1 {
        new_row.push(answer[num_rows as usize - 2][i - 1] + answer[num_rows as usize - 2][i])
    }

    new_row.push(1);
    answer.push(new_row);
    answer
}