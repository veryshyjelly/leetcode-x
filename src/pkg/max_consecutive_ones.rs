#[allow(dead_code)]
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let (mut count, mut max_count) = (0, 0);
    for n in nums {
        if n == 1 {
            count += 1;
        } else {
            max_count = i32::max(max_count, count);
            count = 0;
        }
    }

    i32::max(max_count, count)
}