use fltk::{
    button::Button,
    enums::{FrameType, Shortcut},
    group::{Pack, PackType},
    image::SvgImage,
    prelude::*,
};

use crate::ui::app::CONSTANTS;
use crate::ui::logic;

pub fn menubar() -> Pack {
    let mut menubar = Pack::default().with_size(0, CONSTANTS.rewards_menubar_height);
    menubar.set_type(PackType::Horizontal);

    let mut add_image = SvgImage::from_data(include_str!("../../../assets/menu/plus.svg")).unwrap();
    add_image.scale(24, 24, true, true);

    let mut add_button = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    add_button.set_image(Some(add_image));
    add_button.set_frame(FrameType::FlatBox);

    let mut delete_image =
        SvgImage::from_data(include_str!("../../../assets/menu/minus.svg")).unwrap();
    delete_image.scale(24, 24, true, true);

    let mut delete_button = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    delete_button.set_image(Some(delete_image));
    delete_button.set_frame(FrameType::FlatBox);

    logic(&mut add_button, &mut delete_button);

    menubar.end();
    menubar
}

fn logic<T: ButtonExt + WidgetBase + 'static, U: ButtonExt + WidgetBase + 'static>(
    add_button: &mut T,
    delete_button: &mut U,
) {
    add_button.set_shortcut(Shortcut::from_char('a'));

    add_button.set_callback(|_| {
        println!("Add Pressed!");
    });

    add_button.handle(logic::button_handle);

    delete_button.set_shortcut(Shortcut::from_char('d'));

    delete_button.set_callback(|_| {
        println!("Delete Pressed!");
    });

    delete_button.handle(logic::button_handle);
}
