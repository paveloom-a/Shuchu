use fltk::{
    app,
    button::Button,
    enums::{Event, FrameType, Key, Shortcut},
    image::SvgImage,
    prelude::*,
};

use crate::events;
use crate::ui::app::CONSTANTS;
use crate::ui::logic;

pub fn spend_button() -> Button {
    let mut si =
        SvgImage::from_data(include_str!("../../../../assets/menu/insert-coin.svg")).unwrap();
    si.scale(24, 24, true, true);

    let mut sb = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    sb.set_image(Some(si));
    sb.set_frame(FrameType::FlatBox);
    sb.set_down_frame(FrameType::DownBox);
    sb.set_tooltip("Spend Coins for the Reward");

    logic(&mut sb);

    sb
}

fn logic<T: ButtonExt + WidgetBase + 'static>(sb: &mut T) {
    sb.set_shortcut(Shortcut::from_char('s'));
    sb.set_callback(|_| {});
    sb.handle({
        let mut lock = true;
        let unselect_box = FrameType::FlatBox;
        move |sb, ev| match ev.bits() {
            events::LOCK_THE_SPEND_BUTTON => {
                lock = true;
                sb.set_callback(|_| {});
                if logic::mouse_hovering_widget(sb) {
                    logic::select_inactive(sb);
                }
                true
            }
            events::UNLOCK_THE_SPEND_BUTTON => {
                lock = false;
                sb.set_callback(callback);
                if logic::mouse_hovering_widget(sb) {
                    logic::select_active(sb);
                }
                true
            }
            _ => match ev {
                Event::KeyDown => match app::event_key() {
                    Key::Left => logic::rp_handle_left(sb, 3),
                    Key::Right => logic::rp_handle_right(sb, 4),
                    _ => logic::handle_selection(sb, ev, unselect_box, lock),
                },
                _ => logic::handle_selection(sb, ev, unselect_box, lock),
            },
        }
    });
}

fn callback<T: WidgetBase>(_: &mut T) {
    app::handle_main(events::SPEND_COINS_SEND_PRICE).ok();
}
