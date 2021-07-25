use fltk::{
    app::{self, App},
    enums::{Color, FrameType},
    misc::Tooltip,
};

/// A struct providing access to the application's constants
pub struct Constants {
    pub main_window_width: i32,
    pub main_window_height: i32,
    pub focus_pane_height: i32,
    pub rewards_menubar_height: i32,
    pub rewards_edit_window_width: i32,
    pub rewards_edit_window_height: i32,
    pub scrollbar_width: i32,
}

impl Constants {
    /// Get the default set of the application's constants
    const fn default() -> Constants {
        Constants {
            main_window_width: 340,
            main_window_height: 300,
            focus_pane_height: 60,
            rewards_menubar_height: 30,
            rewards_edit_window_width: 320,
            rewards_edit_window_height: 140,
            scrollbar_width: 17,
        }
    }
}

/// Default set of the application's constants
pub const CONSTANTS: Constants = Constants::default();

/// Create a new App
pub fn new() -> App {
    let app = App::default();
    app::background(255, 255, 255);
    app::set_frame_type(FrameType::BorderBox);

    Tooltip::set_color(Color::BackGround);
    Tooltip::set_hoverdelay(0.0);

    app
}
