#[allow(dead_code)]
pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut answer = vec![];
    let n = nums.len();
    nums.sort_unstable();

    for i in 0..n - 3 {
        if nums[i] * 4 > target {
            break;
        }
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        for j in i + 1..n - 2 {
            if nums[j] * 3 > target - nums[i] {
                continue;
            }
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }
            let (mut left, mut right) = (j + 1, n - 1);
            while left < right {
                let cur_val = nums[i] + nums[j] + nums[left] + nums[right];
                if cur_val == target {
                    answer.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if cur_val > target {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
    }

    answer
}