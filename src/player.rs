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
    jump_tween: Tween,
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
        let jump_tween = Tween::new(&JUMP_WAVEFORM);
        Self {
            state: PlayerState::Idle,
            pos,
            right: true,
            speed_tween,
            jump_tween,
            collider,
            sprites,
        }
    }

    pub fn jump(&mut self) {
        if self.jump_tween.value() < f32::EPSILON {
            self.state = PlayerState::Jumping;
            self.jump_tween.stopped = false;
        }
    }

    pub fn left(&mut self) {
        self.speed_tween.stopped = false;
        self.right = false;
        self.state = PlayerState::Running;
    }

    pub fn right(&mut self) {
        self.speed_tween.stopped = false;
        self.right = true;
        self.state = PlayerState::Running;
    }

    pub fn stop(&mut self) {
        let some_check = false;
        if some_check {
            self.state = PlayerState::Idle;
            self.jump_tween.reset();
        }
        self.speed_tween.reset();
    }

    pub fn update(&mut self, delta: f64) {
        self.speed_tween.update(delta);
        self.jump_tween.update(delta);
        if self.jump_tween.is_over() {
            self.jump_tween.reset();
        }
        let speed_x = if self.right {
            self.speed_tween.value() * 20.0
        } else {
            self.speed_tween.value() * -20.0
        };
        self.pos.x += speed_x * delta as f32;
        let jump_speed = self.jump_tween.value() * -10.0;

        self.pos.y = jump_speed;
    }

    pub fn draw(&self, sprites: &Sprites) {
        match self.state {
            PlayerState::Idle => sprites.draw(&self.sprites[0], self.pos),
            PlayerState::Running => sprites.draw(&self.sprites[0], self.pos),
            PlayerState::Jumping => sprites.draw(&self.sprites[1], self.pos),
            PlayerState::Falling => sprites.draw(&self.sprites[2], self.pos),
        }
    }
}

#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
const JUMP_WAVEFORM: [f32; 90] = [
    0.0,
    16.833333333333332,
    34.166666666666664,
    51.16666666666667,
    64.83333333333333,
    79.16666666666666,
    86.0,
    88.5,
    90.83333333333333,
    91.33333333333333,
    92.33333333333333,
    92.33333333333333,
    92.5,
    92.66666666666666,
    93.16666666666666,
    93.5,
    94.0,
    94.16666666666667,
    94.66666666666667,
    94.83333333333334,
    95.5,
    95.66666666666667,
    95.83333333333334,
    96.33333333333334,
    96.5,
    96.5,
    97.0,
    97.33333333333334,
    98.0,
    98.0,
    98.0,
    98.33333333333333,
    98.66666666666667,
    99.0,
    99.0,
    99.0,
    99.0,
    99.0,
    99.16666666666667,
    99.33333333333333,
    99.5,
    99.33333333333333,
    99.83333333333333,
    100.0,
    99.5,
    100.0,
    100.0,
    100.0,
    100.0,
    100.0,
    99.66666666666667,
    99.16666666666667,
    98.83333333333333,
    98.33333333333333,
    97.33333333333334,
    97.0,
    96.83333333333334,
    96.33333333333334,
    96.16666666666667,
    95.5,
    95.0,
    94.66666666666667,
    94.0,
    93.83333333333333,
    93.33333333333333,
    92.5,
    92.16666666666666,
    91.0,
    90.66666666666666,
    90.0,
    89.33333333333333,
    88.83333333333333,
    87.83333333333333,
    87.16666666666667,
    85.83333333333333,
    84.5,
    83.33333333333334,
    81.33333333333333,
    79.16666666666666,
    76.0,
    73.83333333333333,
    70.66666666666667,
    67.66666666666666,
    66.16666666666666,
    63.0,
    57.333333333333336,
    50.5,
    42.5,
    24.833333333333332,
    0.0,
];
