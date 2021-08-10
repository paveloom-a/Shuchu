// Switch from the console subsystem to the windows subsystem in the release mode
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod channels;
mod events;
mod ui;

use fltk::app;

/// Run the program
fn main() {
    // Application channels
    let channels = channels::Channels::default();

    // Override the system's screen scaling
    for i in 0..app::screen_count() {
        app::set_screen_scale(i, 1.0);
    }

    // 0. App
    let app = ui::app::new();

    // 1. Main Window
    let _w = ui::windows::main(&channels);

    // Hidden Windows

    // 1. Reward Edit Window
    let re_w = ui::windows::rewards_edit(&channels);

    // Start the event loop
    while app.wait() {
        // Retranslate the signals between the windows
        if let Ok(event) = channels.mw.r.try_recv() {
            app::handle_main(event).ok();
        };
        if let Ok(event) = channels.rewards_edit.r.try_recv() {
            app::handle(event, &re_w).ok();
        }
    }
}
