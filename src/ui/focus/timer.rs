use fltk::{
    app,
    button::Button,
    draw,
    enums::{Align, Color, Event, FrameType, Key, LabelType, Shortcut},
    image::SvgImage,
    prelude::*,
};

use crate::events;
use crate::ui::logic;

pub fn timer() -> Button {
    let mut t = Button::default().with_label("00:00:00");
    t.set_label_type(LabelType::None);
    t.set_frame(FrameType::FlatBox);
    t.set_tooltip("Start the timer");

    if let Some(ref p) = t.parent() {
        t.set_pos(p.x() + 10, p.y() + 10);
        t.set_size(110, p.h() - 20);
    }

    t.draw(draw);

    logic(&mut t);

    t
}

fn draw<T: WidgetExt>(b: &mut T) {
    draw::push_clip(b.x(), b.y(), b.w(), b.h());
    draw_label(b);
    draw_image(b);
    draw::pop_clip();
}

fn draw_label<T: WidgetExt>(b: &mut T) {
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

fn draw_image<T: WidgetExt>(b: &mut T) {
    let mut ti = SvgImage::from_data(include_str!("../../../assets/menu/sand-clock.svg")).unwrap();
    ti.scale(24, 24, true, true);
    ti.draw(b.x() + 6, b.y() + 8, 24, 24);
}

fn logic<T: WidgetBase + ButtonExt + 'static>(t: &mut T) {
    t.set_shortcut(Shortcut::from_char('f'));
    t.set_callback(start);
    t.handle({
        let mut counting = false;
        let mut seconds = 0;
        move |t, ev| match ev.bits() {
            events::START_TIMER => {
                if !counting {
                    counting = true;
                    app::add_timeout2(1.0, tick);
                }
                true
            }
            events::TICK => {
                seconds += 1;
                let hours = seconds / 3600;
                let minutes = seconds / 60 % 60;
                let seconds = seconds % 60;
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
            events::STOP_TIMER => {
                counting = false;
                seconds = 0;
                app::remove_timeout2(tick);
                t.set_label("00:00:00");
                true
            }
            _ => {
                if ev == Event::KeyDown {
                    match app::event_key() {
                        Key::Left => logic::fp_handle_left(t, 0),
                        Key::Right => logic::fp_handle_right(t, 0),
                        Key::Tab => logic::handle_tab(t),
                        _ => logic::handle_selection(t, ev, FrameType::FlatBox),
                    }
                } else {
                    logic::handle_selection(t, ev, FrameType::FlatBox)
                }
            }
        }
    });
}

fn start<T: WidgetExt + 'static>(b: &mut T) {
    app::handle_main(events::START_TIMER).ok();
    b.set_tooltip("Stop the timer");
    b.set_callback(stop);
}

fn stop<T: WidgetExt + 'static>(b: &mut T) {
    app::handle_main(events::STOP_TIMER).ok();
    b.set_tooltip("Start the timer");
    b.set_callback(start);
}

fn tick() {
    app::handle_main(events::TICK).ok();
    app::repeat_timeout2(1.0, tick);
}
