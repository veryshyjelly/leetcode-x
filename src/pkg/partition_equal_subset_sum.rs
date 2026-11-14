#[allow(dead_code)]
pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum % 2 != 0 {
        false
    } else {
        subset_sum(nums, sum / 2)
    }
}

pub fn subset_sum(nums: Vec<i32>, sum: i32) -> bool {
    let mut dp = vec![vec![false; nums.len() + 1]; sum as usize + 1];
    dp[0] = vec![true; nums.len() + 1];
    for i in 1..=sum {
        for j in 0..nums.len() {
            if nums[j] <= i {
                dp[i as usize][j + 1] = dp[i as usize][j] || dp[(i - nums[j]) as usize][j];
            } else {
                dp[i as usize][j + 1] = dp[i as usize][j];
            }
        }
    }
    dp[sum as usize][nums.len()]
}