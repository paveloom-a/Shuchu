use fltk::{prelude::*, window::Window};

use super::icon;
use crate::channels::Channels;
use crate::ui::app::CONSTANTS;
use crate::ui::focus;
use crate::ui::rates;
use crate::ui::rewards;

/// Create the Main Window
pub fn main(channels: &Channels) -> Window {
    let mut w = Window::new(
        100,
        100,
        CONSTANTS.main_window_width,
        CONSTANTS.main_window_height,
        "Shuchu",
    );
    w.size_range(
        CONSTANTS.main_window_width,
        CONSTANTS.main_window_height,
        CONSTANTS.main_window_width,
        CONSTANTS.main_window_height + 100,
    );
    w.set_icon(Some(icon()));

    // 1. Focus Pane
    let _fp = focus::pane(channels);

    // 2. Rewards Pane
    let re_p = rewards::pane(channels);

    // 3. Conversion Rates Pane
    let _ra_p = rates::pane();

    w.resizable(&re_p);
    w.end();

    w.show();
    w
}
