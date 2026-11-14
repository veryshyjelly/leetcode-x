#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    let mut res = "";

    let n = s.len();

    let mut dp = vec![vec![false; n]; n];

    let t: Vec<_> = s.chars().collect();

    for i in 0..n - 1 {
        dp[i][i] = true;
        dp[i][i + 1] = t[i] == t[i + 1];
    }
    dp[n - 1][n - 1] = true;

    for i in (0..n).rev() {
        for j in i + 2..n {
            dp[i][j] = dp[i + 1][j - 1] && t[i] == t[j];
        }
        for j in i..n {
            if dp[i][j] && (j - i + 1 > res.len()) {
                res = s.get(i..=j).unwrap().trim();
            }
        }
    }

    res.into()
}