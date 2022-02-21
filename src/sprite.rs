/// name (from the path)
use std::collections::HashMap;

use macroquad::texture::{draw_texture, load_texture, Texture2D};
use macroquad::ui;
use macroquad::{color::Color, color_u8};

const SPRITE_PATHS: &str = include_str!("sprites.txt");

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

    pub fn debug(&mut self) {
        for (name, sprite) in &self.sprites {
            if ui::root_ui().button(None, *name) {
                self.debug = Some(sprite.clone());
            }
        }
        if let Some(sprite) = &self.debug {
            sprite.draw(0.0, 0.0);
        };
    }
}

#[derive(Debug, Clone)]
struct Sprite {
    path: &'static str,
    texture: Texture2D,
}

impl Sprite {
    pub async fn new(path: &'static str) -> Self {
        let texture = load_texture(path).await.unwrap();
        Self { path, texture }
    }

    pub fn draw(&self, x: f32, y: f32) {
        draw_texture(self.texture, x, y, color_u8!(255, 255, 255, 255));
    }
}
