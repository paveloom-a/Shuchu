//! Rates' Menubar provides ways to add / delete / edit the conversion rates.

mod add_button;
mod delete_button;
mod new;

pub use new::new;

use add_button::add_button;
use delete_button::delete_button;
