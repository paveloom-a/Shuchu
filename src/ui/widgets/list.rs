use fltk::{
    app, draw,
    enums::{Align, Color, Event, FrameType, Key},
    group::{Group, Scroll, ScrollType},
    prelude::*,
    widget::Widget,
};
use std::{cell::RefCell, rc::Rc};

type Selected = Rc<RefCell<i32>>;
type Items = Rc<RefCell<Vec<String>>>;

pub trait ScrollExt: GroupExt {
    fn expand(&mut self, n: i32) {
        if let Some(ref mut l) = self.child(0) {
            l.resize(self.x() + 1, self.y() + 1, self.w() - 2, n * 20);
            self.redraw();
        }
    }
}

impl ScrollExt for Scroll {}

pub struct List {
    scroll: Scroll,
    list: Widget,
    selected: Selected,
    items: Items,
}

impl List {
    pub fn default() -> List {
        let mut scroll = Scroll::default();
        scroll.set_frame(FrameType::BorderBox);
        scroll.set_type(ScrollType::Vertical);
        scroll.set_scrollbar_size(17);

        let mut list = Widget::default();
        list.set_color(Color::White);
        list.set_selection_color(Color::DarkBlue);

        scroll.end();

        let mut w = List {
            scroll,
            list,
            selected: Selected::new(RefCell::new(1)),
            items: Items::default(),
        };
        w.draw();
        w.handle(|_, _, _| false);
        w
    }

    pub fn scroll(&self) -> &Scroll {
        &self.scroll
    }

    pub fn parent(&self) -> Option<Group> {
        self.scroll.parent()
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        if width == 0 {
            if let Some(p) = self.parent() {
                self.scroll.set_size(p.w(), height);
            }
        } else {
            self.scroll.set_size(width, height);
        }
    }

    pub fn add(&mut self, s: &'static str) {
        self.items.borrow_mut().push(String::from(s));
        self.scroll.expand(self.items.borrow().len() as i32);
    }

    fn draw(&mut self) {
        let selected = Rc::clone(&self.selected);
        let items = Rc::clone(&self.items);
        self.list.draw(move |l| {
            let lw = f64::from(l.w());
            let dw = draw::width("...");
            let color = draw::get_color();
            for (idx, s) in (0..).zip(items.borrow().iter()) {
                if idx + 1 == *selected.borrow() {
                    draw::draw_rect_fill(l.x(), l.y() + idx * 20, l.w(), 20, l.selection_color());
                    draw::set_draw_color(Color::White);
                    draw::set_font(draw::font(), 16);
                } else {
                    draw::set_font(draw::font(), 16);
                    draw::set_draw_color(Color::Black);
                }

                if draw::width(s) < lw {
                    draw::draw_text2(s, l.x() + 4, l.y() + idx * 20, l.w() - 8, 20, Align::Left);
                } else {
                    let mut n = s.len();
                    while draw::width(&s[..n]) + dw > lw - 8.0 {
                        n -= 1;
                    }
                    draw::draw_text2(
                        &format!("{}...", &s[..n]),
                        l.x() + 4,
                        l.y() + idx * 20,
                        l.w() - 8,
                        20,
                        Align::Left,
                    );
                }
            }
            draw::set_draw_color(color);
        });
    }

    pub fn handle<F: 'static>(&mut self, handle_custom_events: F)
    where
        F: Fn(&mut Scroll, &mut Vec<String>, i32) -> bool,
    {
        self.scroll.handle({
            let selected = Rc::clone(&self.selected);
            let items = Rc::clone(&self.items);
            move |s, ev| match ev {
                Event::Focus => true,
                Event::Push => handle_push(s, &selected, &items),
                Event::KeyDown => handle_keydown(s, &selected, &items),
                _ => handle_custom_events(s, &mut *items.borrow_mut(), ev.bits()),
            }
        });
    }
}

fn handle_push(s: &mut Scroll, selected: &Selected, items: &Items) -> bool {
    s.take_focus().ok();

    if let Some(l) = s.child(0) {
        let mut empty_space_click = true;
        let ys = s.yposition();
        let y = app::event_y();
        let i_max = items.borrow().len() as i32;

        if l.h() < s.h() - 20 {
            for i in 0..i_max {
                if y >= s.y() + i * 20 - ys as i32 && y <= s.y() + (i + 1) * 20 - ys {
                    if *selected.borrow() != i + 1 {
                        *selected.borrow_mut() = i + 1;
                        s.redraw();
                    }
                    empty_space_click = false;
                    break;
                }
            }
        } else {
            let x = app::event_x();

            if x >= s.x() + s.w() - 17 {
                empty_space_click = false;
            } else {
                for i in 0..i_max {
                    if y >= s.y() + i * 20 - ys as i32 && y <= s.y() + (i + 1) * 20 - ys {
                        if *selected.borrow() != i + 1 {
                            *selected.borrow_mut() = i + 1;
                            s.redraw();
                        }
                        empty_space_click = false;
                        break;
                    }
                }
            }
        }

        if empty_space_click && *selected.borrow() != i_max {
            *selected.borrow_mut() = i_max;
            s.redraw();
        }
    }

    true
}

fn handle_keydown(s: &mut Scroll, selected: &Selected, items: &Items) -> bool {
    match app::event_key() {
        Key::Up => {
            if *selected.borrow() == 1 {
                *selected.borrow_mut() = items.borrow().len() as i32;
            } else {
                *selected.borrow_mut() -= 1;
            }
            s.redraw();
            true
        }
        Key::Down => {
            if *selected.borrow() == items.borrow().len() as i32 {
                *selected.borrow_mut() = 1;
            } else {
                *selected.borrow_mut() += 1;
            }
            s.redraw();
            true
        }
        _ => false,
    }
}
