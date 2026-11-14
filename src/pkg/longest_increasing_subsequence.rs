#[allow(dead_code)]
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![1; n];
    for i in (0..n - 1).rev() {
        let key = nums[i];
        for j in i + 1..n {
            if nums[j] > key {
                dp[i] = i32::max(dp[i], 1 + dp[j]);
            }
        }
    }

    dp.into_iter().max().unwrap()
}