//! This module provides the constants.

use fltk::enums::FrameType;

// Numeric constants

/// Main Window's width
pub const MAIN_WINDOW_WIDTH: i32 = 340;
/// Main Window's height
///
/// These 3 pixels are the 1px spacing in Rewards / Rates panes and
/// 1px top and bottom borders of the Scroll widget in the custom List widget
pub const MAIN_WINDOW_HEIGHT: i32 = 300 + 3;
/// Focus Pane's height
pub const FOCUS_PANE_HEIGHT: i32 = 60;
/// Rewards' Menubar's height
pub const REWARDS_MENUBAR_HEIGHT: i32 = 30;
/// Rewards Edit's width
pub const REWARDS_EDIT_WINDOW_WIDTH: i32 = 320;
/// Rewards Edit's height
pub const REWARDS_EDIT_WINDOW_HEIGHT: i32 = 140;

// Default frame types for the buttons

/// Default frame type for boxed buttons
pub const BOXED_BUTTON_FRAME_TYPE: FrameType = FrameType::BorderBox;
/// Default frame type for flat buttons
pub const FLAT_BUTTON_FRAME_TYPE: FrameType = FrameType::FlatBox;
/// Default down frame type for buttons
pub const DOWN_BUTTON_FRAME_TYPE: FrameType = FrameType::DownBox;
