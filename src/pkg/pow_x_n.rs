#[allow(dead_code)]
pub fn my_pow(mut x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if x == 0.0 {
        return 0.0;
    }
    let upon = n < 0;
    let mut n = (n as i64).abs();

    let mut answer = x;
    x = 1.0;

    while n > 1 {
        if n % 2 == 0 {
            answer *= answer;
            n /= 2;
        } else {
            x *= answer;
            n -= 1;
        }
    }

    answer *= x;

    if upon {
        1.0 / answer
    } else {
        answer
    }
}