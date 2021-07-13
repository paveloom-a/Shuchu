use fltk::{enums::FrameType, prelude::*, table::TableRow};

pub fn list() -> TableRow {
    let mut list = TableRow::default();
    list.set_frame(FrameType::BorderBox);

    if let Some(p) = list.parent() {
        list.set_size(0, p.h() - 20);
    }

    list
}
