//! A button, which you can click on to bring the [Rates](super::super::rates) pane.
//!
//! The arrow represents the conversion process between the time and the coins.

use fltk::{app, button::Button, draw, image::SvgImage, prelude::*};

use crate::events;
use crate::ui::constants::{DOWN_BUTTON_FRAME_TYPE, FLAT_BUTTON_FRAME_TYPE};
use crate::ui::logic;

/// Initialize the arrow button
pub fn arrow() -> Button {
    let mut a = Button::default();
    a.set_frame(FLAT_BUTTON_FRAME_TYPE);
    a.set_down_frame(DOWN_BUTTON_FRAME_TYPE);
    a.set_tooltip("Show the Conversion Rates pane");

    // These 14px are the padding. Note that the 48 / 30 ratio is the ratio of the SVG
    a.set_size(48 + 14, 30);

    // Setting the size and position to non-zero values is needed for
    // the widget to appear after re-adding it to the pane. Initial position
    // should not overlap other widgets, since this will lead to visual artifacts
    if let Some(ref p) = a.parent() {
        if let Some(ref lw) = p.child(0) {
            if let Some(ref rw) = p.child(1) {
                replace(&mut a, p, lw, rw);
            }
        }
    }

    a.draw(draw);
    logic(&mut a);

    a
}

/// Draw the arrow button
fn draw<T: WidgetExt>(f: &mut T) {
    // Replace if needed
    if let Some(ref p) = f.parent() {
        if let Some(ref lw) = p.child(0) {
            if let Some(ref rw) = p.child(2) {
                replace(f, p, lw, rw);
            }
        }
    }
    // Setting a clip is a safety measure to limit the region of the drawing
    draw::push_clip(f.x(), f.y(), f.w(), f.h());
    draw_image(f);
    draw::pop_clip();
}

/// Position the arrow in the middle of the two adjacent widgets
fn replace<T: GroupExt, U: WidgetExt, S: WidgetExt>(f: &mut U, p: &T, lw: &S, rw: &S) {
    let lx = lw.x() + lw.w();
    f.set_pos(lx + (rw.x() - lx - f.w()) / 2, p.y() + 15);
}

/// Draw the image of the arrow
fn draw_image<T: WidgetExt>(f: &mut T) {
    let mut ai = SvgImage::from_data(include_str!("../../../assets/menu/arrow.svg")).unwrap();
    // These 14px is the padding
    ai.scale(f.w() - 14, f.h(), true, true);
    ai.draw(f.x() + 7, f.y(), f.w() - 14, f.h());
}

/// Set a callback and a handler for the arrow button
fn logic<T: WidgetBase + ButtonExt>(a: &mut T) {
    // On a callback, show the Rates pane if it's not visible, or,
    // otherwise, hide it. Change the tooltip as appropriate, too
    a.set_callback(|a| {
        if let Some(ref p) = a.parent() {
            if let Some(ref mut c) = p.child(1) {
                if let Some(ref mut w) = p.parent() {
                    if let Some(ref mut re_p) = w.child(1) {
                        if let Some(ref mut ra_p) = w.child(2) {
                            if re_p.visible() {
                                re_p.hide();
                                ra_p.show();
                                a.set_tooltip("Hide the Conversion Rates pane");
                            } else if ra_p.visible() {
                                app::handle_main(events::MAIN_WINDOW_HIDE_THE_PANE).ok();
                                ra_p.hide();
                                a.set_tooltip("Show the Conversion Rates pane");
                            } else {
                                app::handle_main(events::MAIN_WINDOW_SHOW_THE_PANE).ok();
                                ra_p.show();
                                a.set_tooltip("Hide the Conversion Rates pane");
                            }
                            c.set_tooltip("Show the Rewards pane");
                        }
                    }
                }
            }
        }
    });
    // Handle the shortcut and focus / selection events
    a.handle(move |a, ev| {
        logic::handle_shortcut(a, ev.bits(), events::RATES_SHORTCUT)
            || logic::handle_fp_button(a, ev, 1)
    });
}
