#[allow(dead_code)]
pub fn partition(s: String) -> Vec<Vec<String>> {
    if s.len() == 0 {
        return vec![];
    } else if s.len() == 1 {
        return vec![vec![s]];
    }

    let mut answer = vec![];

    for i in (1..s.len()).rev() {
        let (a, b) = s.split_at(i);
        if b != b.chars().rev().collect::<String>() {
            continue;
        }
        let mut rest = partition(a.to_string());
        for r in &mut rest {
            r.push(b.to_string());
        }
        answer.append(&mut rest);
    }

    if s == s.chars().rev().collect::<String>() {
        answer.push(vec![s]);
    }

    answer
}