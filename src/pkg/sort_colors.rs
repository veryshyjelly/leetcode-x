#[allow(dead_code)]
pub fn sort_colors(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut zero_index = 0;
    let mut current_index = n - 1;

    while current_index > zero_index && zero_index < n {
        while zero_index < n && nums[zero_index] == 0 {
            zero_index += 1;
        }
        if zero_index < n {
            if nums[current_index] == 0 {
                nums.swap(current_index, zero_index);
            } else {
                current_index -= 1;
            }
        }
    }

    let mut two_index = n - 1;
    current_index = 0;

    'brother: while current_index < two_index {
        while nums[two_index] == 2 {
            if two_index == 0 {
                break 'brother;
            }
            two_index -= 1;
        }
        if nums[current_index] == 2 {
            nums.swap(current_index, two_index);
        } else {
            current_index += 1;
        }
    }
}