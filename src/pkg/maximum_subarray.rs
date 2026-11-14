#[allow(dead_code)]
pub fn max_sub_array_linear(nums: Vec<i32>) -> i32 {
    let (mut current_sum, mut max_sum) = (0, 0);

    for n in &nums {
        current_sum = i32::max(0, current_sum + n);
        max_sum = i32::max(max_sum, current_sum);
    }

    if max_sum == 0 {
        let mut max_value = nums[0];
        for n in nums {
            max_value = i32::max(max_value, n);
        }
        max_value
    } else {
        max_sum
    }
}

#[allow(dead_code)]
pub fn max_sub_array_divide(nums: Vec<i32>) -> i32 {
    max_sub_array_sum(&nums, 0, nums.len() - 1)
}

pub fn max_sub_array_sum(nums: &Vec<i32>, lo: usize, hi: usize) -> i32 {
    if hi == lo {
        return nums[lo];
    }

    let mid = (lo + hi) / 2;
    let left = max_sub_array_sum(nums, lo, mid);
    let right = max_sub_array_sum(nums, mid + 1, hi);
    let crossing = max_sum_crossing(nums, mid, lo, hi);
    i32::max(crossing, i32::max(left, right))
}

pub fn max_sum_crossing(nums: &Vec<i32>, mid: usize, lo: usize, hi: usize) -> i32 {
    let mut best_left = i32::MIN;
    let mut v = 0;
    for i in (lo..=mid).rev() {
        v += nums[i];
        best_left = i32::max(v, best_left);
    }
    v = 0;
    let mut best_right = i32::MIN;
    for i in (mid + 1..=hi) {
        v += nums[i];
        best_right = i32::max(v, best_right);
    }
    best_right + best_left
}