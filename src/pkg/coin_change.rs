#[allow(dead_code)]
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![i32::MAX / 2; amount as usize + 1];
    dp[0] = 0;

    for i in 1..=coins.len() {
        for j in 1..=amount {
            if coins[i - 1] <= j {
                dp[j as usize] = i32::min(dp[(j - coins[i - 1]) as usize] + 1, dp[j as usize]);
            }
        }
    }
    if dp[amount as usize] >= i32::MAX / 2 {
        -1
    } else {
        dp[amount as usize]
    }
}