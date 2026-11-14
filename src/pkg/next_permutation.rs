#[allow(dead_code)]
pub fn next_permutation(nums: &mut Vec<i32>) {
    let mut i = nums.len() as i32 - 2;

    while i >= 0 && nums[i as usize + 1] <= nums[i as usize] {
        i -= 1;
    }

    if i >= 0 {
        let mut j = nums.len() - 1;
        while nums[j] <= nums[i as usize] {
            j -= 1;
        }
        nums.swap(i as usize, j);
    }
    reverse(nums, (i + 1) as usize);
}

pub fn reverse(nums: &mut Vec<i32>, start: usize) {
    let (mut i, mut j) = (start, nums.len() - 1);
    while i < j {
        nums.swap(i, j);
        i += 1;
        j -= 1;
    }
}