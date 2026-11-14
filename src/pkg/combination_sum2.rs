#[allow(dead_code)]
pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort();
    combination_sum2_helper(&candidates, target, 0)
}

pub fn combination_sum2_helper(candidates: &Vec<i32>, target: i32, skip: usize) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    if target <= 0 {
        return vec![];
    }

    let mut answer = vec![];
    let mut visited = HashSet::new();
    for (i, c) in candidates.iter().skip(skip).enumerate() {
        if visited.contains(c) {
            continue;
        }
        if c.eq(&target) {
            answer.push(vec![target]);
        } else {
            let mut rem = combination_sum2_helper(candidates, target - *c, skip + i + 1);
            for r in &mut rem {
                r.push(*c);
            }
            answer.append(&mut rem);
        }
        visited.insert(*c);
    }

    answer
}