//! This module provides shared behavior between the whole program.

mod get_price;
mod get_text;
mod handle_button;
mod handle_left;
mod handle_lock;
mod handle_right;
mod handle_selection;
mod handle_shortcut;
mod handle_tab;
mod mouse_hovering;

pub use get_price::get_price;
pub use get_text::get_text;
pub use handle_button::{handle_button, handle_fp_button};
pub use handle_lock::handle_lock;
pub use handle_selection::handle_selection;
pub use handle_shortcut::handle_shortcut;

use handle_left::handle_left;
use handle_right::handle_right;
use handle_selection::{select_active, select_inactive};
use handle_tab::handle_tab;
use mouse_hovering::mouse_hovering;
