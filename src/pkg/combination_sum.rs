#[allow(dead_code)]
pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort();
    combination_sum_helper(&candidates, target, -1)
}

pub fn combination_sum_helper(candidates: &Vec<i32>, target: i32, current: i32) -> Vec<Vec<i32>> {
    if target <= 0 {
        return vec![];
    }

    let mut answer = vec![];
    for c in candidates {
        if c < &current {
            continue;
        }
        if c.eq(&target) {
            answer.push(vec![target]);
        } else {
            let mut rem = combination_sum_helper(candidates, target - *c, *c);
            for r in &mut rem {
                r.push(*c);
            }
            answer.append(&mut rem);
        }
    }

    answer
}