#[allow(dead_code)]
pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i64;
    let nums64: Vec<_> = nums.into_iter().map(|x| x as i64).collect();
    let sum: i64 = nums64.iter().sum();
    let squared_sum: i64 = nums64.iter().map(|x| x * x).sum();
    let x = (n * (n + 1)) / 2 - sum;
    let y = (n * (n + 1) * (2 * n + 1)) / 6 - squared_sum;
    let z = y / x;
    vec![((z - x) / 2) as i32, ((x + z) / 2) as i32]
}