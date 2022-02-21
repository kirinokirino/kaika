use std::collections::HashMap;

use macroquad::logging::error;
use macroquad::math::Vec2;
use macroquad::texture::{draw_texture, load_texture, Texture2D};
use macroquad::ui;
use macroquad::{color::Color, color_u8};

use crate::SPRITE_PATHS;

pub struct Sprites {
    sprites: HashMap<&'static str, Sprite>,

    debug: Option<Sprite>,
}

impl Sprites {
    pub async fn new() -> Self {
        let paths = SPRITE_PATHS.split('\n');
        let mut sprites = HashMap::with_capacity(5);
        for path in paths {
            let name = path
                .split('/')
                .last()
                .unwrap()
                .split('.')
                .take(1)
                .last()
                .unwrap();
            let sprite = Sprite::new(path).await;
            sprites.insert(name, sprite);
        }

        Self {
            sprites,
            debug: None,
        }
    }

    pub fn get_sprite_name_by_id(&self, id: usize) -> Option<&'static str> {
        for (i, name) in self.sprites.keys().enumerate() {
            if id == i {
                return Some(*name);
            }
        }
        error!("ERROR: No sprite with id {} found", id);
        None
    }

    pub fn draw(&self, sprite: &str, pos: Vec2) {
        if let Some(sprite) = self.sprites.get(sprite) {
            sprite.draw(pos);
        } else {
            error!("ERROR: Tried to draw a non-existing sprite: {}", sprite);
        }
    }

    pub fn debug(&mut self) {
        for (name, sprite) in &self.sprites {
            if ui::root_ui().button(None, *name) {
                self.debug = Some(sprite.clone());
            }
        }
        if let Some(sprite) = &self.debug {
            sprite.draw(Vec2::new(0.0, 0.0));
        };
    }
}

#[derive(Debug, Clone)]
pub struct Sprite {
    path: &'static str,
    texture: Texture2D,
}

impl Sprite {
    pub async fn new(path: &'static str) -> Self {
        let texture = load_texture(path).await.unwrap();
        Self { path, texture }
    }

    pub fn draw(&self, pos: Vec2) {
        draw_texture(self.texture, pos.x, pos.y, color_u8!(255, 255, 255, 255));
    }
}
