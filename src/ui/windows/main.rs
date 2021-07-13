use fltk::{prelude::*, window::Window};

use super::icon;
use crate::channels::Channels;
use crate::ui::app::CONSTANTS;
use crate::ui::focus;
use crate::ui::rewards;

/// Create the Main Window
pub fn main(_channels: &Channels) -> Window {
    let mut window = Window::new(
        100,
        100,
        CONSTANTS.window_width,
        CONSTANTS.window_height,
        "Shuchu",
    );
    window.set_icon(Some(icon()));

    // 1. Focus Pane
    let _focus_pane = focus::pane();

    // 2. Rewards Pane
    let _rewards_pane = rewards::pane();

    window.end();
    window.show();
    window
}
