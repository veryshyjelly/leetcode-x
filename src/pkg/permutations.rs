#[allow(dead_code)]
pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut temp = vec![];
    let mut seen = vec![false; nums.len()];
    permute_all(&mut nums, &mut res, &mut temp, &mut seen);
    res
}

pub fn permute_all(
    nums: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
    temp: &mut Vec<i32>,
    seen: &mut Vec<bool>,
) {
    if temp.len() == nums.len() {
        res.push(temp.clone());
        return;
    }

    for i in 0..nums.len() {
        if seen[i] {
            continue;
        }
        seen[i] = true;
        temp.push(nums[i]);
        permute_all(nums, res, temp, seen);
        temp.pop();
        seen[i] = false;
    }
}