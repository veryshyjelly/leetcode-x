#[allow(dead_code)]
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut answer = strs[0].len();

    let s = strs[0].clone();

    for t in strs.into_iter().skip(1) {
        for i in 0..answer {
            if s.chars().nth(i) != t.chars().nth(i) {
                answer = i;
                break;
            }
        }
    }

    s.get(0..answer).unwrap().into()
}