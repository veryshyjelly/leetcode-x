#[allow(dead_code)]
pub fn max_product(nums: Vec<i32>) -> i32 {
    use std::mem::swap;
    let mut answer = nums[0];
    let (mut max_positive, mut max_negative) = (answer, answer);

    for n in nums {
        if n < 0 {
            swap(&mut max_positive, &mut max_negative);
        }
        max_positive = i32::max(n, n * max_positive);
        max_negative = i32::min(n, n * max_negative);
        answer = i32::max(answer, max_positive);
    }

    answer
}