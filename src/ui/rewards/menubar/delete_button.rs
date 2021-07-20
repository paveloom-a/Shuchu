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

pub fn delete_button() -> Button {
    let mut di = SvgImage::from_data(include_str!("../../../../assets/menu/minus.svg")).unwrap();
    di.scale(24, 24, true, true);

    let mut db = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    db.set_image(Some(di));
    db.set_frame(FrameType::FlatBox);
    db.set_down_frame(FrameType::DownBox);
    db.set_tooltip("Delete the Reward");

    logic(&mut db);

    db
}

fn logic<T: ButtonExt + WidgetBase + 'static>(db: &mut T) {
    db.set_shortcut(Shortcut::from_char('d'));
    db.set_callback(|_| {});
    db.handle({
        let mut lock = true;
        let unselect_box = FrameType::FlatBox;
        move |db, ev| match ev.bits() {
            events::LOCK_THE_DELETE_A_REWARD_BUTTON => {
                lock = true;
                db.set_callback(|_| {});
                if logic::mouse_hovering_widget(db) {
                    logic::select_inactive(db);
                }
                true
            }
            events::UNLOCK_THE_DELETE_A_REWARD_BUTTON => {
                lock = false;
                db.set_callback(callback);
                if logic::mouse_hovering_widget(db) {
                    logic::select_active(db);
                }
                true
            }
            _ => match ev {
                Event::KeyDown => match app::event_key() {
                    Key::Left => logic::rp_handle_left(db, 1),
                    Key::Right => logic::rp_handle_right(db, 1),
                    _ => logic::handle_selection(db, ev, unselect_box, lock),
                },
                _ => logic::handle_selection(db, ev, unselect_box, lock),
            },
        }
    });
}

fn callback<T: WidgetBase>(_: &mut T) {
    app::handle_main(events::DELETE_A_REWARD).ok();
}
