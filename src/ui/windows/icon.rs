//! Load a Window Icon.

use fltk::image::PngImage;

/// Load a Window Icon
pub fn icon() -> PngImage {
    PngImage::from_data(include_bytes!("../../../assets/shuchu-32.png")).unwrap()
}
