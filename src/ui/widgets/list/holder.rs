use fltk::{enums::Event, group::Pack, prelude::*};

use super::{Item, Items};

pub struct Holder {
    holder: Pack,
}

impl Holder {
    pub fn default() -> Self {
        Holder {
            holder: Pack::default(),
        }
    }

    pub fn handle<F: FnMut(&mut Pack, Event) -> bool + 'static>(&mut self, cb: F) {
        self.holder.handle(cb)
    }

    fn resize(h: &mut Pack, n: usize) {
        if let Some(ref mut s) = h.parent() {
            h.resize(s.x() + 1, s.y() + 1, s.w() - 2, n as i32 * 20);
        }
    }

    pub fn add_to(h: &mut Pack, string: String, items: &Items) {
        Self::resize(h, items.len() + 1);

        h.begin();
        let item = Item::new(string);
        h.end();

        h.add(item.frame());
        items.push(item);

        h.redraw()
    }

    pub fn add(&mut self, string: String, items: &Items) {
        Self::add_to(&mut self.holder, string, items)
    }

    pub fn remove(h: &mut Pack, index: usize, items: &mut Items) {
        h.remove(items.index(index).frame());
        items.remove(index);
        Self::resize(h, items.len());
        if let Some(ref mut s) = h.parent() {
            s.redraw();
        }
    }
}
