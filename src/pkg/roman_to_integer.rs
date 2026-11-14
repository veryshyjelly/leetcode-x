#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    let mut current_value = 0;
    let mut final_value = 0;

    for c in s.chars() {
        match c {
            'I' => {
                current_value = 1;
                final_value += 1
            }
            'V' => {
                if current_value == 1 {
                    final_value += 3;
                } else {
                    final_value += 5
                }
                current_value = 0
            }
            'X' => {
                if current_value == 1 {
                    final_value += 8;
                    current_value = 0;
                } else {
                    current_value = 10;
                    final_value += 10;
                }
            }
            'L' => {
                if current_value == 10 {
                    final_value += 30
                } else {
                    final_value += 50
                }
                current_value = 0;
            }
            'C' => {
                if current_value == 10 {
                    final_value += 80;
                    current_value = 0;
                } else {
                    current_value = 100;
                    final_value += 100;
                }
            }
            'D' => {
                if current_value == 100 {
                    final_value += 300
                } else {
                    final_value += 500
                }
                current_value = 0;
            }
            'M' => {
                if current_value == 100 {
                    final_value += 800;
                } else {
                    final_value += 1000
                }
                current_value = 0;
            }
            _ => {}
        }
    }

    final_value
}