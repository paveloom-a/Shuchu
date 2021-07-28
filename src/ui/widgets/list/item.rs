use fltk::{
    draw,
    enums::{Align, Color, FrameType},
    frame::Frame,
    prelude::*,
};
use std::{
    cell::{Ref, RefCell, RefMut},
    ops::Range,
    rc::Rc,
};

use crate::ui::app::CONSTANTS;
const TEXT_PADDING: i32 = 4;

type ItemString = Rc<RefCell<String>>;
type ItemIsSelected = Rc<RefCell<bool>>;
type ItemLabel = Rc<RefCell<String>>;

pub struct Item {
    frame: Frame,
    string: ItemString,
    selected: ItemIsSelected,
    label: ItemLabel,
}

impl Item {
    pub fn new(string: String) -> Self {
        let mut frame = Frame::default();
        frame.set_frame(FrameType::FlatBox);

        let selected = ItemIsSelected::default();
        let label = ItemLabel::default();

        if let Some(ref h) = frame.parent() {
            frame.set_size(h.w(), 20);
            Self::update_label_for(&mut frame, &label, string.clone());
            frame.draw({
                let selected = Rc::clone(&selected);
                let label = Rc::clone(&label);
                move |f| {
                    let color = draw::get_color();
                    if *selected.borrow() {
                        draw::set_draw_color(Color::White);
                        draw::set_font(draw::font(), 16);
                    } else {
                        draw::set_font(draw::font(), 16);
                        draw::set_draw_color(Color::Black);
                    }
                    draw::draw_text2(
                        &*label.borrow(),
                        f.x() + TEXT_PADDING,
                        f.y(),
                        f.w() - 2 * TEXT_PADDING - CONSTANTS.scrollbar_width,
                        f.h(),
                        Align::Left,
                    );
                    draw::set_draw_color(color);
                }
            });
        }

        Item {
            frame,
            string: ItemString::new(RefCell::new(string)),
            selected,
            label,
        }
    }

    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    /// Get the y coordinate of the item
    pub fn y(&self) -> i32 {
        self.frame.y()
    }

    /// Get the height of the item
    pub fn h(&self) -> i32 {
        self.frame.h()
    }

    fn string(&self) -> Ref<String> {
        self.string.borrow()
    }

    fn string_mut(&self) -> RefMut<String> {
        self.string.borrow_mut()
    }

    pub fn clone(&self) -> String {
        self.string().clone()
    }

    pub fn selected(&self) -> bool {
        *self.selected.borrow()
    }

    fn selected_mut(&self) -> RefMut<bool> {
        self.selected.borrow_mut()
    }

    pub fn find(&self, pat: char) -> Option<usize> {
        self.string().find(pat)
    }

    pub fn set(&mut self, string: String) {
        *self.string_mut() = string.clone();
        self.update_label(string);
    }

    fn update_label(&mut self, string: String) {
        Self::update_label_for(&mut self.frame, &self.label, string)
    }

    fn update_label_for(f: &mut Frame, l: &ItemLabel, string: String) {
        draw::set_font(draw::font(), 16);

        let fw = f64::from(f.w());
        let sw = draw::width(&string);
        let dw = draw::width("...");
        let cw = fw - f64::from(TEXT_PADDING) - f64::from(CONSTANTS.scrollbar_width);

        if cw > 0.0 {
            if sw < cw {
                f.set_tooltip("");
                *l.borrow_mut() = string;
            } else {
                let mut n = string.len();
                while draw::width(&string[..n]) + dw > cw {
                    n -= 1;
                }
                f.set_tooltip(&string);
                *l.borrow_mut() = string[..n].to_string() + "...";
            }
        }
    }

    pub fn index(&self, index: Range<usize>) -> Ref<str> {
        Ref::map(self.string(), |items| &items[index])
    }

    pub fn select(&mut self) {
        *self.selected_mut() = true;
        self.frame.set_color(Color::DarkBlue);
    }

    pub fn unselect(&mut self) {
        *self.selected_mut() = false;
        self.frame.set_color(Color::BackGround);
    }
}
