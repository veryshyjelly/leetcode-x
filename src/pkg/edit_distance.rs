#[allow(dead_code)]
pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1: Vec<_> = word1.chars().collect();
    let word2: Vec<_> = word2.chars().collect();

    let (n, m) = (word1.len(), word2.len());

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = i as i32;
    }
    for j in 0..=m {
        dp[0][j] = j as i32;
    }

    for i in 0..n {
        for j in 0..m {
            if word1[i] == word2[j] {
                dp[i + 1][j + 1] = dp[i][j];
            } else {
                dp[i + 1][j + 1] =
                    i32::min(dp[i][j] + 1, i32::min(dp[i][j + 1] + 1, dp[i + 1][j] + 1));
            }
        }
    }

    dp[n][m]
}