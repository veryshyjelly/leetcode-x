#[allow(dead_code)]
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let (mut tortoise, mut hare) = (nums[0] as usize, nums[nums[0] as usize] as usize);
    while tortoise != hare {
        tortoise = nums[tortoise] as usize;
        hare = nums[nums[hare] as usize] as usize;
    }

    tortoise = 0;
    while tortoise != hare {
        tortoise = nums[tortoise] as usize;
        hare = nums[hare] as usize;
    }

    hare as i32
}