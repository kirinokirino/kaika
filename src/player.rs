use std::fmt::Display;

use macroquad::math::Vec2;

use crate::collider::Collider;
use crate::sprite::Sprites;
use crate::tween::{Tween, TWEEN_PERIOD};

#[derive(PartialEq, Debug)]
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
    pub collider: Collider,
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

    fn init(&mut self) {}

    pub fn jump(&mut self) {
        if self.jump_tween.stopped {
            self.state = PlayerState::Jumping;
            self.jump_tween.stopped = false;
        }
    }

    pub fn jump_stop(&mut self) {
        if self.state == PlayerState::Jumping {
            if self.jump_tween.time > JUMP_START_OFFSET as f32 * TWEEN_PERIOD {
                self.state = PlayerState::Falling;
                self.jump_tween.set_offset(JUMP_END_OFFSET);
            }
        }
    }

    pub fn left(&mut self) {
        self.speed_tween.stopped = false;
        self.right = false;

        if self.state != PlayerState::Jumping {
            self.state = PlayerState::Running;
        }
    }

    pub fn right(&mut self) {
        self.speed_tween.stopped = false;
        self.right = true;

        if self.state != PlayerState::Jumping {
            self.state = PlayerState::Running;
        }
    }

    pub fn stop(&mut self) {
        // FIXME sholud be collision with the ground
        let some_check = self.jump_tween.stopped;
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
        let jump_speed = self.jump_tween.value() * -5.0;

        self.pos.y = jump_speed;

        if self.jump_tween.time > JUMP_END_OFFSET as f32 * TWEEN_PERIOD {
            self.state = PlayerState::Falling;
        }
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

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Player {:?}, jump_tween: {}",
            self.state, self.jump_tween
        ))
    }
}

const JUMP_START_OFFSET: usize = 10;
const JUMP_END_OFFSET: usize = 54;
#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
const JUMP_WAVEFORM: [f32; 60] = [
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
    92.5,
    93.16666666666666,
    93.5,
    94.0,
    94.83333333333334,
    95.5,
    96.0,
    97.33333333333334,
    98.0,
    98.33333333333333,
    98.66666666666667,
    99.0,
    99.5,
    100.0,
    99.66666666666667,
    99.16666666666667,
    98.83333333333333,
    97.33333333333334,
    97.0,
    96.83333333333334,
    96.83333333333334,
    96.33333333333334,
    96.33333333333334,
    96.16666666666667,
    96.16666666666667,
    95.5,
    95.5,
    95.0,
    95.0,
    94.66666666666667,
    94.66666666666667,
    94.0,
    94.0,
    93.33333333333333,
    93.33333333333333,
    92.5,
    92.5,
    92.16666666666666,
    92.16666666666666,
    91.0,
    91.0,
    90.66666666666666,
    90.66666666666666,
    90.0,
    90.0,
    80.0,
    65.0,
    45.0,
    25.0,
    0.0,
];
