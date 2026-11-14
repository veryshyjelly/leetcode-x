#[allow(dead_code)]
pub fn majority_element(mut nums: Vec<i32>) -> Vec<i32> {
    let (mut candi1, mut candi2) = (0, 0);
    let (mut count1, mut count2) = (0, 0);

    nums.iter().for_each(|&x| {
        if count1 == 0 && x != candi2 {
            count1 = 1;
            candi1 = x;
        } else if count2 == 0 && x != candi1 {
            count2 = 1;
            candi2 = x;
        } else if x == candi1 {
            count1 += 1;
        } else if x == candi2 {
            count2 += 1;
        } else {
            count1 -= 1;
            count2 -= 1;
        }
    });

    let mut answer = vec![];
    count1 = 0;
    count2 = 0;
    let n = nums.len();
    for x in nums {
        if candi1 == x {
            count1 += 1;
        } else if candi2 == x {
            count2 += 1;
        }
    }
    if count1 > n / 3 {
        answer.push(candi1);
    }
    if count2 > n / 3 {
        answer.push(candi2);
    }
    answer
}