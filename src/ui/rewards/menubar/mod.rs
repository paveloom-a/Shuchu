//! Rewards' Menubar provides ways for the user to add / delete / edit / spend money on rewards.

mod add_button;
mod delete_button;
mod divider;
mod edit_button;
mod new;
mod spend_button;

pub use new::new;

use add_button::add_button;
use delete_button::delete_button;
use divider::divider;
use edit_button::edit_button;
use spend_button::spend_button;
