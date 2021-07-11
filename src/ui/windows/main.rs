use fltk::{prelude::*, window::Window};

use super::icon;
use crate::channels::Channels;
use crate::ui::app::CONSTANTS;

/// Create the Main Window
pub fn main(_channels: &Channels) -> Window {
    let mut window = Window::new(
        100,
        100,
        CONSTANTS.window_min_width,
        CONSTANTS.window_min_height,
        "Shuchu",
    );
    window.set_icon(Some(icon()));
    window.size_range(
        CONSTANTS.window_min_width,
        CONSTANTS.window_min_height,
        0,
        0,
    );
    window.make_resizable(true);

    window.end();
    window.show();
    window
}
