extern crate sdl2;

use self::sdl2::rect::Rect;
use config;

pub fn rect_centered(width: i32,
                     height: i32,
                     horizontal_offset: i32,
                     vertical_offset: i32)
                     -> Rect {
    Rect::new(((config::WINDOW_WIDTH as i32 / 2i32) - (width / 2i32) + horizontal_offset as i32),
              ((config::WINDOW_HEIGHT as i32 / 2i32) - (height / 2i32) + vertical_offset as i32),
              width as u32,
              height as u32)
}

pub fn point_colliding_rect(x: i32, y: i32, rect: &Rect) -> bool {
    if x >= rect.x() && y >= rect.y() && x <= (rect.x() + rect.width() as i32) &&
       y <= (rect.y() + rect.height() as i32) {
        true
    } else {
        false
    }
}
