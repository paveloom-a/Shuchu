//! List's Item is a wrapper around a string in the list.
//!
//! It manages its associated widget, label (the displayed string), and a selection state.

use fltk::{
    draw,
    enums::{Align, Color, FrameType},
    frame::Frame,
    prelude::*,
};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use super::{ITEM_HEIGHT, SCROLLBAR_WIDTH, TEXT_PADDING};

/// An item in the List
pub struct Item {
    /// Associated widget
    frame: Frame,
    /// Contents of the item
    string: Rc<RefCell<String>>,
    /// Selection state
    selected: Rc<RefCell<bool>>,
    /// Visible (within the width of the list) part of the [`string`](Item#structfield.string)
    label: Rc<RefCell<String>>,
}

impl Item {
    /// Initialize a new item
    pub fn new(string: String) -> Self {
        // Create an item with default values
        let mut item = Item {
            frame: Frame::default(),
            string: Rc::new(RefCell::new(string)),
            selected: Rc::<RefCell<bool>>::default(),
            label: Rc::<RefCell<String>>::default(),
        };

        item.frame.set_frame(FrameType::FlatBox);

        // If the item is a child of the Holder
        if let Some(ref h) = item.frame.parent() {
            // Change the width of the frame and update the label
            item.frame.set_size(h.w(), ITEM_HEIGHT);
            item.update_label();

            // Set a custom `draw` function
            item.frame.draw({
                let selected = Rc::clone(&item.selected);
                let label = Rc::clone(&item.label);
                move |f| {
                    // Saving the draw color and reapplying it in the end is a safety measure
                    let color = draw::get_color();

                    // If the item is selected
                    if *selected.borrow() {
                        draw::set_draw_color(Color::White);
                    } else {
                        draw::set_draw_color(Color::Black);
                    }

                    // Draw the text within the padding box
                    draw::set_font(draw::font(), 16);
                    draw::draw_text2(
                        &*label.borrow(),
                        f.x() + TEXT_PADDING,
                        f.y(),
                        f.w() - 2 * TEXT_PADDING - SCROLLBAR_WIDTH,
                        f.h(),
                        Align::Left,
                    );

                    draw::set_draw_color(color);
                }
            });
        }

        item
    }

    /// Get the frame of the item
    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    /// Get the contents of the item
    pub fn string(&self) -> Ref<String> {
        self.string.borrow()
    }

    /// Clone the contents of the item
    pub fn clone(&self) -> String {
        self.string().clone()
    }

    /// Get the selection state of the item
    pub fn selected(&self) -> bool {
        *self.selected.borrow()
    }

    /// Check if the item is partially or completely hidden
    pub fn hidden(&self, completely: bool) -> bool {
        self.frame.parent().map_or(false, |ref p| {
            // Get the List's Scroll
            p.parent().map_or(false, |ref s| {
                // Return `true` if
                if completely {
                    self.frame.y() > s.y() + s.h() // The top border is hidden below, or
                    || self.frame.y() + self.frame.h() < s.y() // The bottom border is hidden above
                } else {
                    self.frame.y() < s.y() // The top border is hidden above, or
                    || self.frame.y() + self.frame.h() > s.y() + s.h() // The bottom border is hidden below
                }
            })
        })
    }

    /// Set the contents of the item to the passed string
    pub fn set(&mut self, string: String) {
        *self.string.borrow_mut() = string;
        self.update_label();
    }

    /// Update the label (the visible part of the string) of the item
    fn update_label(&mut self) {
        // Get the copy of the item's contents
        let string = self.string().clone();

        // Setting the font size is crucial for the calculation of the visible width
        draw::set_font(draw::font(), 16);

        // Width of the frame, but
        let aw = f64::from(self.frame.w())
            // Minus the width of the text padding (meaning, from the right side)
            - f64::from(TEXT_PADDING)
            // Minus the scrollbar width
            - f64::from(SCROLLBAR_WIDTH);
        // Visible width of the string
        let sw = draw::width(&string);
        // Visible width of the "..." string
        let dw = draw::width("...");

        // If the available width is positive (this is a safety measure)
        if aw > 0.0 {
            // If the string's width is shorter than the available width
            if sw < aw {
                // Disable the tooltip
                self.frame.set_tooltip("");
                // Set the label to the string
                *self.label.borrow_mut() = string;
            // Otherwise,
            } else {
                // Shorten the string until it (plus the dots) fits
                let mut n = self.string().len();
                while draw::width(&string[..n]) + dw > aw {
                    n -= 1;
                }
                // Set the tooltip to the string
                self.frame.set_tooltip(&string);
                // Set the label to the shortened string
                *self.label.borrow_mut() = self.string()[..n].to_string() + "...";
            }
        }
    }

    /// Select the item
    pub fn select(&mut self) {
        *self.selected.borrow_mut() = true;
        self.frame.set_color(Color::DarkBlue);
    }

    /// Unselect the item
    pub fn unselect(&mut self) {
        *self.selected.borrow_mut() = false;
        self.frame.set_color(Color::BackGround);
    }
}
