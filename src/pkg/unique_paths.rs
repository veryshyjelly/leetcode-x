#[allow(dead_code)]
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut answer = 1;
    let (m, n) = (
        i32::max(m - 1, n - 1) as u128,
        i32::min(m - 1, n - 1) as u128,
    );
    for i in m + 1..=m + n {
        answer *= i;
    }
    for i in 1..=n {
        answer /= i;
    }
    answer as i32
}