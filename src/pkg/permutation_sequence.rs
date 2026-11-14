#[allow(dead_code)]
pub fn get_permutation(n: i32, k: i32) -> String {
    let n = n as usize;
    let mut numbers: Vec<_> = (1..=n).collect();
    let mut factorial = vec![1; n + 1];
    for i in 1..=n {
        factorial[i] = i * factorial[i - 1];
    }

    let mut res = String::new();

    let mut k = k as usize - 1;

    for i in 1..=n {
        let index = k / factorial[n - i];
        res.push_str(&numbers[index].to_string());
        numbers.remove(index);
        k -= (index) * factorial[n - i];
    }

    res
}