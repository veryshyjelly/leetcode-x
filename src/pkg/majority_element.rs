#[allow(dead_code)]
pub fn majority_element(mut nums: Vec<i32>) -> i32 {
    let (mut major, mut count) = (0, 0);
    for n in nums {
        if count == 0 {
            count = 1;
            major = n;
        } else if major == n {
            count += 1;
        } else {
            count -= 1;
        }
    }

    major
}