use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut answer = vec![];
    let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
    for (i, n) in nums.into_iter().enumerate() {
        while !queue.is_empty() && queue.back().unwrap().1 <= n {
            queue.pop_back();
        }
        queue.push_back((i, n));
        if queue.front().unwrap().0 + k as usize == i {
            queue.pop_front();
        }
        answer.push(queue.front().unwrap().1)
    }
    answer.into_iter().skip(k as usize - 1).collect()
}