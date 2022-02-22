use macroquad::math::Vec2;

use crate::collider::Collider;
use crate::sprite::Sprites;
use crate::tween::Tween;

enum PlayerState {
    Idle,
    Running,
    Jumping,
    Falling,
}

pub struct Player {
    state: PlayerState,
    pub pos: Vec2,
    right: bool,
    speed_tween: Tween,
    collider: Collider,
    pub sprites: Vec<String>,
}

impl Player {
    pub fn new(pos: Vec2, collider: Collider, sprites: &[String]) -> Self {
        let sprites = sprites.to_vec();
        debug_assert!(!sprites.is_empty());
        let speed_tween = Tween::new(&[
            0.0, 25.0, 38.0, 50.0, 60.0, 70.0, 80.0, 84.0, 87.0, 90.0, 93.0, 96.0, 97.0, 98.0,
            99.0, 100.0,
        ]);
        Self {
            state: PlayerState::Idle,
            pos,
            right: true,
            speed_tween,
            collider,
            sprites,
        }
    }

    pub fn jump(&mut self) {}

    pub fn left(&mut self) {
        self.speed_tween.stopped = false;
        self.right = false;
    }

    pub fn right(&mut self) {
        self.speed_tween.stopped = false;
        self.right = true;
    }

    pub fn stop(&mut self) {
        self.speed_tween.reset();
    }

    pub fn update(&mut self, delta: f64) {
        self.speed_tween.update(delta);
        let speed_x = if self.right {
            self.speed_tween.value() * 20.0
        } else {
            self.speed_tween.value() * -20.0
        };
        self.pos.x += speed_x * delta as f32;
    }

    pub fn draw(&self, sprites: &Sprites) {
        sprites.draw(&self.sprites[0], self.pos);
    }
}
