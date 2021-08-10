//! A button, which you can click on to bring the [Rewards](super::super::rewards) pane.
//!
//! The widget also keeps the number of coins the user has, and updates it if necessary.

use fltk::{
    app,
    button::Button,
    draw,
    enums::{Align, Color, LabelType},
    image::SvgImage,
    prelude::*,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::constants::{DOWN_BUTTON_FRAME_TYPE, FLAT_BUTTON_FRAME_TYPE};
use crate::ui::logic;

/// Initialize the coins button
pub fn coins(channels: &Channels) -> Button {
    // Setting the label, but in the meantime disabling
    // its default drawing allows for the custom drawing
    let mut c = Button::default().with_label("99999");
    c.set_label_type(LabelType::None);

    c.set_frame(FLAT_BUTTON_FRAME_TYPE);
    c.set_down_frame(DOWN_BUTTON_FRAME_TYPE);
    c.set_tooltip("Hide the Rewards pane");

    // This widget changes in size depending on the number of (digits in) the coins
    resize(&mut c);
    c.draw(draw);
    logic(&mut c, channels);

    c
}

/// Draw the coins button
fn draw<T: WidgetExt>(b: &mut T) {
    // Setting a clip is a safety measure to limit the region of the drawing
    draw::push_clip(b.x(), b.y(), b.w(), b.h());
    draw_label(b);
    draw_image(b);
    draw::pop_clip();
}

/// Resize the button depending on the size of its label
fn resize<T: WidgetExt>(b: &mut T) {
    let (lw, _) = b.measure_label();
    if let Some(ref p) = b.parent() {
        b.set_pos(p.x() + p.w() - lw - 60, p.y() + 10);
        b.set_size(lw + 50, p.h() - 20);
    }
}

/// Draw the button's label
fn draw_label<T: WidgetExt>(b: &mut T) {
    // Saving the draw color and reapplying it in the end is a safety measure
    let color = draw::get_color();

    draw::set_font(draw::font(), 16);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(
        &b.label(),
        b.x() + 33,
        b.y() + 1,
        b.w() - 40,
        b.h(),
        Align::Right,
    );

    draw::set_draw_color(color);
}

/// Draw the image of the coins
fn draw_image<T: WidgetExt>(b: &mut T) {
    let mut ci = SvgImage::from_data(include_str!("../../../assets/menu/coins.svg")).unwrap();
    ci.scale(24, 24, true, true);
    ci.draw(b.x() + 6, b.y() + 8, 24, 24);
}

/// Set a callback and a handler for the coins button
fn logic<T: WidgetBase + ButtonExt + 'static>(c: &mut T, channels: &Channels) {
    // On a callback, show the Rewards pane if it's not visible, or,
    // otherwise, hide it. Change the tooltip as appropriate, too
    c.set_callback(|c| {
        if let Some(ref p) = c.parent() {
            if let Some(ref mut a) = p.child(2) {
                if let Some(ref mut w) = p.parent() {
                    if let Some(ref mut re_p) = w.child(1) {
                        if let Some(ref mut ra_p) = w.child(2) {
                            if ra_p.visible() {
                                ra_p.hide();
                                re_p.show();
                                c.set_tooltip("Hide the Rewards pane");
                            } else if re_p.visible() {
                                app::handle_main(events::MAIN_WINDOW_HIDE_THE_PANE).ok();
                                re_p.hide();
                                c.set_tooltip("Show the Rewards pane");
                            } else {
                                app::handle_main(events::MAIN_WINDOW_SHOW_THE_PANE).ok();
                                re_p.show();
                                c.set_tooltip("Hide the Rewards pane");
                            }
                            a.set_tooltip("Show the Conversion Rates pane");
                        }
                    }
                }
            }
        }
    });
    // Handle the custom events, shortcut, and focus / selection events
    c.handle({
        // Receive coins
        let r_coins = channels.rewards_receive_coins.r.clone();
        move |c, ev| {
            let bits = ev.bits();
            // In case of the
            match bits {
                // 'Spend the coins' event, receive the price
                events::SPEND_COINS_RECEIVE_PRICE => r_coins.try_recv().map_or(false, |price| {
                    // Get the total number of coins
                    c.label().parse::<f64>().map_or(false, |coins| {
                        // Calculate the diff, rounding it to the second digit in mantissa
                        let diff = ((coins - price) * 100.0).round() / 100.0;
                        // Update the total number of coins
                        c.set_label(&diff.to_string());
                        // Send a signal to Rewards to delete the currently selected reward
                        app::handle_main(events::DELETE_A_REWARD).ok();
                        // Resize the coins button if the number of digits changed
                        resize(c);
                        // Redraw the pane, so that the `arrow`'s
                        // position gets updated too if needed
                        if let Some(ref mut p) = c.parent() {
                            p.redraw();
                        }
                        true
                    })
                }),
                // `Check the affordability` event
                events::CHECK_AFFORDABILITY_RECEIVE_PRICE => {
                    // Receive the price
                    r_coins.try_recv().map_or(false, |price| {
                        // Get the total number of coins
                        c.label().parse::<f64>().map_or(false, |coins| {
                            // If the reward is affordable
                            if price <= coins {
                                // Unlock the spend button
                                app::handle_main(events::UNLOCK_THE_SPEND_BUTTON).ok();
                            // Otherwise,
                            } else {
                                // Lock it
                                app::handle_main(events::LOCK_THE_SPEND_BUTTON).ok();
                            }
                            true
                        })
                    })
                }
                // Otherwise, handle the shortcut and focus / selection events
                _ => {
                    logic::handle_shortcut(c, bits, events::REWARDS_SHORTCUT)
                        || logic::handle_fp_button(c, ev, 2)
                }
            }
        }
    });
}
