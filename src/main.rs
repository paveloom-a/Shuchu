// Switch from the console subsystem to the windows subsystem in the release mode
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod channels;
mod events;
mod ui;

use fltk::app;

fn main() {
    // Application channels
    let channels = channels::Channels::default();

    // Override the system's screen scaling
    for i in 0..app::screen_count() {
        app::set_screen_scale(i, 1.0);
    }

    // 0. App
    let app = ui::app::new();

    // 1. Window
    let _window = ui::windows::main(&channels);

    // Start the event loop
    while app.wait() {}
}
