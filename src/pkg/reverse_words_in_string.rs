#[allow(dead_code)]
pub fn reverse_words(s: String) -> String {
    let mut x = s.trim().split_whitespace().collect::<Vec<&str>>();
    x.reverse();
    x.join(" ")
}