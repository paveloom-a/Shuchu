use fltk::{
    app,
    button::Button,
    draw,
    enums::{Align, Color, Event, FrameType, Key, LabelType, Shortcut},
    image::SvgImage,
    prelude::*,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::logic;

pub fn coins(channels: &Channels) -> Button {
    let mut c = Button::default().with_label("99999");
    c.set_label_type(LabelType::None);
    c.set_frame(FrameType::FlatBox);
    c.set_down_frame(FrameType::DownBox);
    c.set_tooltip("Hide the Rewards pane");

    resize_button(&mut c);
    c.draw(draw);

    logic(&mut c, channels);

    c
}

fn draw<T: WidgetExt>(b: &mut T) {
    resize_button(b);
    draw::push_clip(b.x(), b.y(), b.w(), b.h());
    draw_label(b);
    draw_image(b);
    draw::pop_clip();
}

fn resize_button<T: WidgetExt>(b: &mut T) {
    let (lw, _) = b.measure_label();
    if let Some(ref p) = b.parent() {
        b.set_pos(p.x() + p.w() - lw - 60, p.y() + 10);
        b.set_size(lw + 50, p.h() - 20);
    }
}

fn draw_label<T: WidgetExt>(b: &mut T) {
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

fn draw_image<T: WidgetExt>(b: &mut T) {
    let mut ci = SvgImage::from_data(include_str!("../../../assets/menu/coins.svg")).unwrap();
    ci.scale(24, 24, true, true);
    ci.draw(b.x() + 6, b.y() + 8, 24, 24);
}

fn logic<T: WidgetBase + ButtonExt + 'static>(c: &mut T, channels: &Channels) {
    c.set_shortcut(Shortcut::from_char('c'));
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
    c.handle({
        let r_coins = channels.rewards_receive_coins.r.clone();
        move |c, ev| match ev {
            Event::KeyDown => match app::event_key() {
                Key::Left => logic::fp_handle_left(c, 1),
                Key::Right => logic::fp_handle_right(c, 1),
                Key::Tab => logic::handle_tab(c),
                _ => logic::handle_active_selection(c, ev, FrameType::FlatBox),
            },
            _ => match ev.bits() {
                events::SPEND_COINS_RECEIVE_PRICE => r_coins.try_recv().map_or(false, |price| {
                    c.label().parse::<f64>().map_or(false, |coins| {
                        let diff = ((coins - price) * 100.0).round() / 100.0;
                        c.set_label(&diff.to_string());
                        app::handle_main(events::DELETE_A_REWARD).ok();
                        if let Some(ref mut p) = c.parent() {
                            p.redraw();
                        }
                        true
                    })
                }),
                events::CHECK_AFFORDABILITY_RECEIVE_PRICE => {
                    r_coins.try_recv().map_or(false, |price| {
                        c.label().parse::<f64>().map_or(false, |coins| {
                            if price <= coins {
                                app::handle_main(events::UNLOCK_THE_SPEND_BUTTON).ok();
                            } else {
                                app::handle_main(events::LOCK_THE_SPEND_BUTTON).ok();
                            }
                            true
                        })
                    })
                }
                _ => logic::handle_active_selection(c, ev, FrameType::FlatBox),
            },
        }
    });
}
