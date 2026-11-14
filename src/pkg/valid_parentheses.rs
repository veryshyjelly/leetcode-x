#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for c in s.chars() {
        if is_closer(c) {
            if closer(stack.pop().unwrap_or('*')) != c {
                return false;
            }
        } else {
            stack.push(c);
        }
    }

    stack.len() == 0
}

pub fn is_closer(c: char) -> bool {
    match c {
        ')' => true,
        ']' => true,
        '}' => true,
        _ => false,
    }
}

pub fn closer(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => '*',
    }
}