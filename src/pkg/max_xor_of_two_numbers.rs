use std::collections::HashSet;

#[allow(dead_code)]
pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut mask = 0;
    for i in (0..32).rev() {
        mask = mask | (1 << i);
        let mut set = HashSet::new();
        for n in &nums {
            set.insert(n & mask);
        }
        let temp = max | (1 << i);
        for prefix in set.iter() {
            if set.contains(&(temp ^ prefix)) {
                max = temp;
                break;
            }
        }
    }
    max
}