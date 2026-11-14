#[allow(dead_code)]
pub fn my_atoi(s: String) -> i32 {
    let mut answer = 0;

    let str = s.trim();

    let sign = str.chars().nth(0).unwrap() == '-';

    let max: i64;
    if sign {
        max = 1 << 31;
    } else {
        max = (1 << 31) - 1;
    }

    let skipping;
    if str.chars().nth(0).unwrap() == '-' || str.chars().nth(0).unwrap() == '+' {
        skipping = 1;
    } else {
        skipping = 0;
    }

    for x in str.chars().skip(skipping) {
        if !x.is_numeric() {
            break;
        }
        answer = answer * 10 + x.to_digit(10).unwrap() as i64;
        if answer > max {
            if sign {
                return -max as i32;
            } else {
                return max as i32;
            }
        }
    }

    answer as i32 * if sign { -1 } else { 1 }
}