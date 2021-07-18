use fltk::prelude::*;

use crate::ui::widgets::List;

pub fn list() -> List {
    let mut l = List::default();

    if let Some(ref p) = l.parent() {
        l.set_size(0, p.h() - 20);
    }

    l.add("5/m");
    l.add("0.1/s");

    l
}
