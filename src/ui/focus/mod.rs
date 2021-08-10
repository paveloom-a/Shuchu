//! Focus pane is the primary pane, providing access to the focus / conversion process.
//!
//! It provides ways for the user to:
//! - start the time-to-coins conversion process;
//! - see how long this process is running;
//! - see the total number of coins;
//! - get access to the [Rates](super::rates) pane;
//! - get access to the [Rewards](super::rewards) pane

mod arrow;
mod coins;
mod pane;
mod timer;

pub use pane::pane;

use arrow::arrow;
use coins::coins;
use timer::timer;
