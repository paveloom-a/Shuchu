//! Rates Pane is a secondary pane that provides access to conversion rates.
//!
//! It gives ways for the user to:
//! - select an existing conversion rate;
//! - create a new conversion rate;
//! - delete the existing conversion rate

mod list;
mod menubar;
mod pane;

pub use pane::pane;
