use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use macroquad::prelude::*;

use crate::audio::Audio;
use crate::camera::{top_down_camera_controls, Camera};
use crate::collider::Collider;
use crate::sprite::{Sprite, Sprites};
use crate::static_layers::{StaticEntity, StaticLayers};

#[allow(clippy::module_name_repetitions)]
pub enum WorldState {
    Menu,
    Play,
    Edit,
    Debug,
}

pub struct World {
    pub state: WorldState,
    sprites: Sprites,
    audio: Audio,

    static_layers: StaticLayers,

    time: Time,
    main_camera: Camera,

    // Edit
    chosen_entity: Option<&'static str>,
}

impl World {
    pub fn new(audio: Audio, sprites: Sprites) -> Self {
        Self {
            state: WorldState::Debug,
            audio,
            sprites,

            static_layers: StaticLayers::new(),

            time: Time::default(),
            main_camera: Camera::new(),

            chosen_entity: None,
        }
    }

    pub fn setup(&mut self) {
        self.add_static_entity();
        self.add_static_entity();
    }

    pub fn add_static_entity(&mut self) {
        let sprite = self
            .sprites
            .get_sprite_name_by_id(0)
            .expect("no sprite with id 0");
        let collider = Collider::new(Vec2::new(0.0, 0.0), 100.0, 100.0);
        let entity = StaticEntity::new(sprite, collider);
        self.static_layers.add_entity(0, entity);
    }

    pub fn input(&mut self) {
        let _w = is_key_down(KeyCode::W) || is_key_down(KeyCode::Comma);
        let _s = is_key_down(KeyCode::S) || is_key_down(KeyCode::O);
        let _a = is_key_down(KeyCode::A);
        let _d = is_key_down(KeyCode::D) || is_key_down(KeyCode::E);

        if is_key_pressed(KeyCode::Space) {
            self.state = match self.state {
                WorldState::Play => WorldState::Debug,
                WorldState::Debug => WorldState::Edit,
                WorldState::Edit => WorldState::Play,
                _ => todo!(),
            }
        }

        match self.state {
            WorldState::Menu => (),
            WorldState::Play => (),
            WorldState::Edit => self.edit_input(),
            WorldState::Debug => self.debug_input(),
        }

        if is_key_down(KeyCode::LeftControl) {
            top_down_camera_controls(&mut self.main_camera);
        }
    }

    fn edit_input(&mut self) {
        let mouse = self.main_camera.mouse_world_position();

        let lmb = is_mouse_button_pressed(MouseButton::Left);

        if let Some(entity) = self.chosen_entity {
            if lmb {
                let entity = StaticEntity::new(entity, Collider::new(mouse, 100.0, 100.0));
                self.static_layers.add_entity(0, entity);
            } else {
                self.sprites.draw(entity, mouse);
            }
        }
    }

    fn debug_input(&mut self) {
        let mut line = 1u8;
        let font_size = 24.0;
        let line_height = font_size + 0.0;
        let padding = 10.0;
        let color = color_u8!(0, 0, 0, 255);

        let camera_info = true;
        let mouse_info = true;
        let key_info = true;

        if camera_info {
            let camera = self.main_camera;
            draw_text(
                &format!(
                    "target: {}, zoom: {:?}, view_port: {:?}",
                    camera.target,
                    camera.zoom,
                    camera.viewport_size(),
                ),
                padding,
                f32::from(line).mul_add(line_height, padding),
                font_size,
                color,
            );
            line += 1;
        }

        if mouse_info {
            let mouse = self.main_camera.mouse_world_position();
            draw_text(
                &format!("mouse: {:?}, mouse_world: {}", mouse_position(), mouse),
                padding,
                f32::from(line).mul_add(line_height, padding),
                font_size,
                color,
            );
            line += 1;
        }

        if key_info {
            for key_code in (0..1000).map(From::from) {
                if is_key_down(key_code) {
                    let text = format!("{:?}", key_code);
                    draw_text(
                        &text,
                        padding,
                        f32::from(line).mul_add(line_height, padding),
                        font_size,
                        color,
                    );
                    line += 1;
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.update_time(get_time());
        let delta = self.time.delta;
        self.main_camera.update();
    }

    fn update_time(&mut self, time: f64) {
        self.time = Time {
            delta: time - self.time.overall,
            overall: get_time(),
        };
    }

    pub fn draw(&mut self) {
        // Camera space, render game objects
        let zoom = vec2(self.main_camera.zoom.x, -self.main_camera.zoom.y);
        set_camera(&Camera2D {
            target: self.main_camera.target,
            rotation: -self.main_camera.rotation.to_degrees(),
            zoom,
            ..Camera2D::default()
        });

        match self.state {
            WorldState::Debug => self.debug_draw(),
            WorldState::Play => self.play_draw(),
            WorldState::Edit => self.edit_draw(),
            _ => todo!(),
        }
    }

    fn edit_draw(&mut self) {
        self.static_layers.draw(&self.sprites);

        set_default_camera();
        if let Some(chosen) = self.sprites.ui() {
            self.chosen_entity = Some(chosen);
        }
    }

    fn play_draw(&self) {
        let Rect { x, y, w, h } = self.main_camera.viewport_rect();
        draw_rectangle_lines(x, y, w, h, w / 100.0, color_u8!(50, 120, 100, 100));
        let (width, height) = (screen_width(), screen_height());
        let (center_x, center_y) = (self.main_camera.target.x, self.main_camera.target.y);
        let top_left_x = center_x - width;
        let top_left_y = center_y - height;
        draw_rectangle_lines(
            top_left_x,
            top_left_y,
            width * 2.0,
            height * 2.0,
            50.0,
            color_u8!(50, 120, 100, 100),
        );

        draw_rectangle(-5.0, -5.0, 10.0, 10.0, color_u8!(180, 180, 180, 255));

        self.static_layers.draw(&self.sprites);

        set_default_camera();
    }

    fn debug_draw(&mut self) {
        set_default_camera();
        self.audio.debug();
        self.sprites.debug();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_level(&self) {
        todo!()
    }
    #[cfg(target_arch = "wasm32")]
    pub fn save_level(&self) {}
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct Time {
    delta: f64,
    overall: f64,
}
