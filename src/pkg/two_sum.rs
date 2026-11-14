use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indexes: HashMap<i32, usize> = HashMap::new();

    for (i, n) in nums.into_iter().enumerate() {
        if indexes.contains_key(&(target - n)) {
            return vec![i as i32, *indexes.get(&(target - n)).unwrap() as i32];
        }
        indexes.insert(n, i);
    }

    vec![]
}