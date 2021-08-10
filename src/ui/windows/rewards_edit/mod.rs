//! Rewards Edit window provides a way to add / edit items in the [Rewards](super::super::rewards)
//! list.
//!
//! This secondary window is called by the 'Add a Reward' and 'Edit the Reward' buttons in the
//! Rewards' Menubar.

mod buttons;
mod cancel;
mod coins;
mod new;
mod ok;
mod reward;

pub use buttons::buttons;
pub use coins::coins;
pub use new::new;
pub use reward::reward;

use cancel::cancel;
use ok::ok;
