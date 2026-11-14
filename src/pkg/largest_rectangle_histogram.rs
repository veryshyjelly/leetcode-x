#[allow(dead_code)]
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut stack = vec![(-1, n as i32)];
    let mut next_smaller = vec![(0, 0); n];

    for i in (0..n).rev() {
        let cur = heights[i];
        while stack.last().unwrap().0 >= cur {
            stack.pop();
        }
        next_smaller[i] = *stack.last().unwrap();
        stack.push((cur, i as i32));
    }

    stack = vec![(-1, -1)];
    let mut previous_smaller = vec![(0, 0); n];
    for i in 0..n {
        let cur = heights[i];
        while stack.last().unwrap().0 >= cur {
            stack.pop();
        }
        previous_smaller[i] = *stack.last().unwrap();
        stack.push((cur, i as i32));
    }

    let mut answer = 0;
    for i in 0..n {
        let width = next_smaller[i].1 - previous_smaller[i].1 - 1;
        answer = i32::max(heights[i] * width, answer)
    }

    answer
}