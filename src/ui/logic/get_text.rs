//! Get the text from the Rewards' Item's string.

/// Get the text from the Rewards' Item's string
pub fn get_text(string: &str) -> Option<String> {
    if let Some(rb_i) = string.find(')') {
        if rb_i + 2 <= string.len() {
            Some(string[(rb_i + 2)..].to_string())
        } else {
            None
        }
    } else {
        None
    }
}
