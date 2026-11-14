#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let (mut remote, mut car) = (0, 0);
    let n = nums.len();

    while car < n {
        if nums[remote] != nums[car] {
            nums[remote + 1] = nums[car];
            remote = remote + 1;
        }
        car = car + 1;
    }

    remote as i32 + 1
}