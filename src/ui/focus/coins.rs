use fltk::{
    app,
    button::Button,
    draw,
    enums::{Align, Color, Event, FrameType, Key, LabelType, Shortcut},
    image::SvgImage,
    prelude::*,
};

use crate::ui::app::CONSTANTS;
use crate::ui::logic;

pub fn coins() -> Button {
    let mut coins = Button::default().with_label("99999");
    coins.set_label_type(LabelType::None);
    coins.set_frame(FrameType::FlatBox);

    draw_button(&mut coins);
    coins.draw(draw);

    logic(&mut coins);

    coins
}

fn draw<T: WidgetExt>(b: &mut T) {
    draw::push_clip(b.x(), b.y(), b.w(), b.h());
    draw_button(b);
    draw_label(b);
    draw_image(b);
    draw::pop_clip();
}

fn draw_button<T: WidgetExt>(b: &mut T) {
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
        b.x() + 30,
        b.y(),
        b.w() - 40,
        b.h(),
        Align::Right,
    );

    draw::set_draw_color(color);
}

fn draw_image<T: WidgetExt>(b: &mut T) {
    let mut coins_image =
        SvgImage::from_data(include_str!("../../../assets/menu/coins.svg")).unwrap();
    coins_image.scale(24, 24, true, true);
    coins_image.draw(b.x() + 6, b.y() + 7, 24, 24);
}

fn logic<T: WidgetBase + ButtonExt + 'static>(c: &mut T) {
    c.set_shortcut(Shortcut::from_char('c'));
    c.set_callback(|c| {
        if let Some(ref p) = c.parent() {
            if let Some(ref mut w) = p.parent() {
                if let Some(ref mut re_p) = w.child(1) {
                    if let Some(ref mut ra_p) = w.child(2) {
                        if ra_p.visible() {
                            ra_p.hide();
                            re_p.show();
                        } else if re_p.visible() {
                            w.resize(w.x(), w.y(), w.w(), 10 + CONSTANTS.focus_pane_height + 10);
                            re_p.hide();
                        } else {
                            w.resize(w.x(), w.y(), w.w(), CONSTANTS.window_height);
                            re_p.show();
                        }
                    }
                }
            }
        }
    });
    c.handle(|c, ev| {
        if ev == Event::KeyDown {
            match app::event_key() {
                Key::Left => logic::fp_handle_left(c, 1),
                Key::Right => logic::fp_handle_right(c, 1),
                Key::Tab => logic::handle_tab(c),
                _ => logic::handle_selection(c, ev),
            }
        } else {
            logic::handle_selection(c, ev)
        }
    });
}
