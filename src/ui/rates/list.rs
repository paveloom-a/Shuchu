use fltk::prelude::*;

use crate::ui::widgets::List;

pub fn list() -> List {
    let mut list = List::default();

    if let Some(ref p) = list.parent() {
        list.set_size(0, p.h() - 20);
    }

    list.add("5/m");
    list.add("0.1/s");

    list
}
