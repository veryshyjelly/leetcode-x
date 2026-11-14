#[allow(dead_code)]
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;

    if nums.len() < 3 {
        return vec![];
    }

    nums.sort();

    if nums[0] > 0 {
        return vec![];
    }

    let mut my_map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        my_map.insert(nums[i], i);
    }

    let mut res = vec![];

    let mut first = 0;
    while first < nums.len() - 2 {
        if nums[first] > 0 {
            break;
        }
        let mut second = first + 1;

        while second < nums.len() - 1 {
            let target = -(nums[first] + nums[second]);
            if my_map.contains_key(&target) && my_map.get(&target).unwrap() > &second {
                res.push(vec![nums[first], nums[second], target]);
            }
            second = *my_map.get(&nums[second]).unwrap() + 1;
        }
        first = *my_map.get(&nums[first]).unwrap() + 1;
    }

    res
}