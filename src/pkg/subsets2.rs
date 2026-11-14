#[allow(dead_code)]
pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut res = vec![];
    subsets_helper(&nums, 0, &mut vec![], &mut res);
    res
}

pub fn subsets_helper(nums: &Vec<i32>, idx: usize, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    res.push(subset.to_vec());

    for i in idx..nums.len() {
        if i != idx && nums[i] == nums[i - 1] {
            continue;
        }
        subset.push(nums[i]);
        subsets_helper(nums, i + 1, subset, res);
        subset.pop();
    }
}