//! List widget is a rework of the standard [`Browser`](fltk::browser::Browser) widget that removes
//! some annoyances (like the focus box) and allows for some customization (items' tooltips,
//! shortened labels, custom selection, etc). The user interface is similar where possible.

mod holder;
mod item;
mod items;
mod selected;
mod widget;

pub use holder::Holder;
pub use item::Item;
pub use items::Items;
pub use selected::Selected;
pub use widget::List;

/// Height of an item in the list
const ITEM_HEIGHT: i32 = 20;
/// Width of the scrollbar
const SCROLLBAR_WIDTH: i32 = 17;
/// The text padding for the label
const TEXT_PADDING: i32 = 4;
