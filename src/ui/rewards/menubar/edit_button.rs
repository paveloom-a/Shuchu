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

pub fn edit_button() -> Button {
    let mut ei = SvgImage::from_data(include_str!("../../../../assets/menu/pencil.svg")).unwrap();
    ei.scale(24, 24, true, true);

    let mut eb = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    eb.set_image(Some(ei));
    eb.set_frame(FrameType::FlatBox);
    eb.set_down_frame(FrameType::DownBox);
    eb.set_tooltip("Edit the Reward");

    logic(&mut eb);

    eb
}

fn logic<T: ButtonExt + WidgetBase + 'static>(eb: &mut T) {
    eb.set_shortcut(Shortcut::from_char('e'));
    eb.set_callback(|_| {});
    eb.handle({
        let mut lock = true;
        let unselect_box = FrameType::FlatBox;
        move |eb, ev| match ev.bits() {
            events::LOCK_THE_EDIT_A_REWARD_BUTTON => {
                lock = true;
                eb.set_callback(|_| {});
                if logic::mouse_hovering_widget(eb) {
                    logic::select_inactive(eb);
                }
                true
            }
            events::UNLOCK_THE_EDIT_A_REWARD_BUTTON => {
                lock = false;
                eb.set_callback(callback);
                if logic::mouse_hovering_widget(eb) {
                    logic::select_active(eb);
                }
                true
            }
            _ => match ev {
                Event::KeyDown => match app::event_key() {
                    Key::Left => logic::rp_handle_left(eb, 2),
                    Key::Right => logic::rp_handle_right(eb, 3),
                    _ => logic::handle_selection(eb, ev, unselect_box, lock),
                },
                _ => logic::handle_selection(eb, ev, unselect_box, lock),
            },
        }
    });
}

fn callback<T: WidgetBase>(_: &mut T) {
    app::handle_main(events::EDIT_A_REWARD_SEND_ITEM).ok();
}
