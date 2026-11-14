#[allow(dead_code)]
pub fn longest_consecutive(nums_: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut nums: HashSet<i32> = HashSet::new();
    for n in nums_ {
        nums.insert(n);
    }
    let mut answer = 0;

    while nums.len() != 0 {
        let ele = nums.iter().next().unwrap().clone();
        nums.remove(&ele);
        let mut count = 1;
        let mut temp = ele - 1;
        while nums.contains(&temp) {
            nums.remove(&temp);
            temp -= 1;
            count += 1;
        }
        temp = ele + 1;
        while nums.contains(&temp) {
            nums.remove(&temp);
            temp += 1;
            count += 1;
        }
        answer = i32::max(answer, count);
    }

    answer
}