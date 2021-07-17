use fltk::{prelude::*, window::Window};

use super::icon;
use crate::channels::Channels;
use crate::ui::app::CONSTANTS;
use crate::ui::focus;
use crate::ui::rates;
use crate::ui::rewards;

/// Create the Main Window
pub fn main(channels: &Channels) -> Window {
    let mut window = Window::new(
        100,
        100,
        CONSTANTS.main_window_width,
        CONSTANTS.main_window_height,
        "Shuchu",
    );
    window.size_range(
        CONSTANTS.main_window_width,
        CONSTANTS.main_window_height,
        CONSTANTS.main_window_width,
        CONSTANTS.main_window_height + 100,
    );
    window.set_icon(Some(icon()));

    // 1. Focus Pane
    let _focus_pane = focus::pane();

    // 2. Rewards Pane
    let rewards_pane = rewards::pane(channels);

    // 3. Conversion Rates Pane
    let _rates_pane = rates::pane();

    window.resizable(&rewards_pane);
    window.end();

    window.show();
    window
}
