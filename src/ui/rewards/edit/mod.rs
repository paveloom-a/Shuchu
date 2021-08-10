//! This module provides widgets for the [Rewards Edit](mod@crate::ui::windows::rewards_edit) window.

mod buttons;
mod cancel;
mod coins;
mod ok;
mod reward;

pub use buttons::buttons;
pub use coins::coins;
pub use reward::reward;

use cancel::cancel;
use ok::ok;
