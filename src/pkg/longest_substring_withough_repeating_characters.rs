#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;
    let mut indexes: HashMap<char, usize> = HashMap::new();

    let characs: Vec<_> = s.chars().collect();

    let (mut answer, mut last_deleted) = (0, -1);

    for (i, c) in s.chars().enumerate() {
        if indexes.contains_key(&c) {
            answer = i32::max(answer, i as i32 - last_deleted - 1);
            let now_deleting = *indexes.get(&c).unwrap();
            for j in (last_deleted + 1) as usize..=now_deleting {
                indexes.remove(&characs[j]);
            }
            last_deleted = now_deleting as i32;
        }
        indexes.insert(c, i);
    }

    answer = i32::max(answer, indexes.len() as i32);

    answer
}