//! This module provides the app initialization function.

use fltk::{
    app::{self, App},
    enums::{Color, FrameType},
    misc::Tooltip,
};

/// Initialize the app
pub fn new() -> App {
    let app = App::default();
    app::background(255, 255, 255);
    app::set_frame_type(FrameType::BorderBox);

    Tooltip::set_color(Color::BackGround);
    Tooltip::set_hoverdelay(0.0);

    app
}
