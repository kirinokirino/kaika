use std::convert::From;
use std::fmt::{self, Display};

use macroquad::prelude::*;

#[derive(Clone, PartialEq)]
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

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.pos.x + self.width / 2.0,
            self.pos.y + self.height / 2.0,
        )
    }

    pub fn draw(&self, offset: Vec2) {
        let Rect { x, y, w, h } = self.rect();
        draw_rectangle(
            x + offset.x,
            y + offset.y,
            w,
            h,
            color_u8!(255, 100, 100, 100),
        );
    }

    pub fn is_zero(&self) -> bool {
        self.width + self.height < 1.0
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

impl From<[f32; 4]> for Collider {
    fn from(t: [f32; 4]) -> Self {
        Self::new(Vec2::new(t[0], t[1]), t[2], t[3])
    }
}
