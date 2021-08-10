//! Rewards Pane is a secondary pane that provides access to rewards.
//!
//! It gives ways for the user to:
//! - select an existing reward;
//! - create a new reward;
//! - delete the existing reward;
//! - spend coins on the selected reward

pub mod edit;

mod list;
mod menubar;
mod pane;

pub use pane::pane;
