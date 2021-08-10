//! Return [`true`] if mouse hovers the widget.

use fltk::{app, prelude::*};

/// Return [`true`] if the mouse hovers the widget
pub fn mouse_hovering<T: WidgetBase>(w: &T) -> bool {
    app::event_x() >= w.x()
        && app::event_x() <= w.x() + w.w()
        && app::event_y() >= w.y()
        && app::event_y() <= w.y() + w.h()
}
