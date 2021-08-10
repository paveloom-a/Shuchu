//! Parse the price in a Rewards' Item's string.

/// Parse the price in a Rewards' Item's string
pub fn get_price(string: &str) -> Option<f64> {
    if let Some(rb_i) = string.find(')') {
        if let Ok(price) = string[1..rb_i].parse::<f64>() {
            Some(price)
        } else {
            None
        }
    } else {
        None
    }
}
