#[allow(dead_code)]
pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        return "1".to_string();
    }

    let prev = count_and_say(n - 1);
    let mut answer = String::new();

    let mut iterator = prev.chars();
    let mut current = iterator.next().unwrap();

    'outer: loop {
        let mut count = 1;
        loop {
            if let Some(next_char) = iterator.next() {
                if next_char == current {
                    count += 1;
                } else {
                    answer.push_str(&format!("{}", count));
                    answer.push(current);
                    current = next_char;
                    break;
                }
            } else {
                if count != 0 {
                    answer.push_str(&format!("{}", count));
                    answer.push(current);
                }
                break 'outer;
            }
        }
    }

    answer
}