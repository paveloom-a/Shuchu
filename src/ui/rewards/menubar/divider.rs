//! Divider divides elements in the menubar.

use fltk::{frame::Frame, image::SvgImage, prelude::*};

/// Initialize a divider
pub fn divider() -> Frame {
    let mut di = SvgImage::from_data(include_str!("../../../../assets/menu/divider.svg")).unwrap();
    // Note that the ratio is the same as in the SVG
    di.scale(8, 24, true, true);

    let mut d = Frame::default().with_size(10, 0);
    d.set_image(Some(di));

    // Disabling the visible focus allows for `Left` / `Right` navigation to ignore this frame
    d.visible_focus(false);

    d
}
