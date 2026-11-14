pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    use std::collections::HashMap;
    let mut s_count = HashMap::new();
    for c in s.chars() {
        s_count.insert(c, s_count.get(&c).unwrap_or(&0) + 1);
    }
    let mut t_count = HashMap::new();
    for c in t.chars() {
        t_count.insert(c, t_count.get(&c).unwrap_or(&0) + 1);
    }

    for c in s_count.keys() {
        if s_count.get(c) != t_count.get(c) {
            return false;
        }
    }
    true
}