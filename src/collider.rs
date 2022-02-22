use std::fmt::{self, Display};

use macroquad::prelude::*;

pub struct Collider {
    pub pos: Vec2,
    width: f32,
    height: f32,
}

impl Collider {
    #[must_use]
    pub const fn new(pos: Vec2, width: f32, height: f32) -> Self {
        Self { pos, width, height }
    }

    pub fn rect(&self) -> Rect {
        Rect {
            x: self.pos.x,
            y: self.pos.y,
            w: self.width,
            h: self.height,
        }
    }

    pub fn draw(&self) {
        let Rect { x, y, w, h } = self.rect();
        draw_rectangle(x, y, w, h, color_u8!(255, 100, 100, 100));
    }
}

impl Display for Collider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Collider {{ x:{}, y:{}, w:{}, h:{} }} ",
            self.pos.x, self.pos.y, self.width, self.height
        )
    }
}
