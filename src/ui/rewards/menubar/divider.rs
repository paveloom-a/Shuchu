use fltk::{frame::Frame, image::SvgImage, prelude::*};

pub fn divider() -> Frame {
    let mut di = SvgImage::from_data(include_str!("../../../../assets/menu/divider.svg")).unwrap();
    di.scale(8, 24, true, true);

    let mut d = Frame::default().with_size(10, 0);
    d.set_image(Some(di));

    d
}
