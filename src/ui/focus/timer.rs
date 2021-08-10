//! A button, which represents a timer. Clicking on it starts the ticking.
//!
//! While it ticks, the coins get generated. Clicking on it again stops and resets the timer.

use fltk::{
    app,
    button::Button,
    draw,
    enums::{Align, Color, LabelType},
    image::SvgImage,
    prelude::*,
};

use crate::events;
use crate::ui::constants::{DOWN_BUTTON_FRAME_TYPE, FLAT_BUTTON_FRAME_TYPE};
use crate::ui::logic;

/// Initialize the timer
pub fn timer() -> Button {
    // Setting the label, but in the meantime disabling
    // its default drawing allows for the custom drawing
    let mut t = Button::default().with_label("00:00:00");
    t.set_label_type(LabelType::None);

    t.set_frame(FLAT_BUTTON_FRAME_TYPE);
    t.set_down_frame(DOWN_BUTTON_FRAME_TYPE);
    t.set_tooltip("Start the timer");

    // The timer is the only one among the three widgets
    // in the pane which change neither position nor size
    if let Some(p) = t.parent() {
        t.set_pos(p.x() + 10, p.y() + 10);
        t.set_size(110, p.h() - 20);
    }

    t.draw(draw);

    logic(&mut t);

    t
}

/// Draw the timer
fn draw<T: WidgetExt>(b: &mut T) {
    // Setting a clip is a safety measure to limit the region of the drawing
    draw::push_clip(b.x(), b.y(), b.w(), b.h());
    draw_label(b);
    draw_image(b);
    draw::pop_clip();
}

/// Draw the timer's label
fn draw_label<T: WidgetExt>(b: &mut T) {
    // Saving the draw color and reapplying it in the end is a safety measure
    let color = draw::get_color();

    draw::set_font(draw::font(), 16);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(
        &b.label(),
        b.x() + 32,
        b.y() + 1,
        b.w() - 40,
        b.h(),
        Align::Right,
    );

    draw::set_draw_color(color);
}

/// Draw the timer's icon
fn draw_image<T: WidgetExt>(b: &mut T) {
    let mut ti = SvgImage::from_data(include_str!("../../../assets/menu/sand-clock.svg")).unwrap();
    ti.scale(24, 24, true, true);
    ti.draw(b.x() + 6, b.y() + 8, 24, 24);
}

/// Set a callback and a handler for the timer
fn logic<T: WidgetBase + ButtonExt + 'static>(t: &mut T) {
    // The default callback is set to start the ticking
    t.set_callback(start);
    // Handle the custom events, shortcut, and focus / selection events
    t.handle({
        // Is the timer counting?
        let mut counting = false;
        // How many seconds does it have at the moment?
        let mut seconds = 0;
        move |t, ev| {
            let bits = ev.bits();
            // In case of the
            match bits {
                // 'Start the timer` event
                events::START_TIMER => {
                    // If it's not counting already
                    if !counting {
                        // Start the counting
                        counting = true;
                        app::add_timeout2(1.0, tick);
                    }
                    true
                }
                // 'Tick' event
                events::TICK => {
                    seconds += 1;

                    // Calculate the integer parts of the time
                    let hours = seconds / 3600;
                    let minutes = seconds / 60 % 60;
                    let seconds = seconds % 60;

                    // Update the label, applying the `00:00:00` format to these numbers
                    t.set_label(&format!(
                        "{}:{}:{}",
                        if hours > 9 {
                            format!("{}", hours)
                        } else {
                            format!("0{}", hours)
                        },
                        if minutes > 9 {
                            format!("{}", minutes)
                        } else {
                            format!("0{}", minutes)
                        },
                        if seconds > 9 {
                            format!("{}", seconds)
                        } else {
                            format!("0{}", seconds)
                        }
                    ));

                    true
                }
                // 'Stop the timer' event
                events::STOP_TIMER => {
                    // Stop the counting
                    counting = false;
                    seconds = 0;
                    app::remove_timeout2(tick);
                    // Reset the label
                    t.set_label("00:00:00");
                    true
                }
                // Otherwise, handle the shortcut and focus / selection events
                _ => {
                    logic::handle_shortcut(t, bits, events::TIMER_SHORTCUT)
                        || logic::handle_fp_button(t, ev, 0)
                }
            }
        }
    });
}

/// Start the ticking
fn start<T: WidgetExt + 'static>(b: &mut T) {
    app::handle_main(events::START_TIMER).ok();
    b.set_tooltip("Stop the timer");
    b.set_callback(stop);
}

/// Stop the ticking
fn stop<T: WidgetExt + 'static>(b: &mut T) {
    app::handle_main(events::STOP_TIMER).ok();
    b.set_tooltip("Start the timer");
    b.set_callback(start);
}

/// Tick
fn tick() {
    app::handle_main(events::TICK).ok();
    app::repeat_timeout2(1.0, tick);
}
