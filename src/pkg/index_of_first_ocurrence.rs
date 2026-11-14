#[allow(dead_code)]
pub fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(ans) = haystack.find(&needle) {
        return ans as i32;
    }
    -1
}