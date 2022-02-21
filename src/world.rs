use macroquad::prelude::*;

use crate::camera::{top_down_camera_controls, Camera};

pub struct World {
    time: Time,
    main_camera: Camera,
}

impl World {
    pub fn new() -> Self {
        Self {
            time: Time::default(),
            main_camera: Camera::new(),
        }
    }
    pub fn input(&mut self) {
        let lmb = is_mouse_button_pressed(MouseButton::Left);
        let W = is_key_down(KeyCode::W) || is_key_down(KeyCode::Comma);
        let S = is_key_down(KeyCode::S) || is_key_down(KeyCode::O);
        let A = is_key_down(KeyCode::A);
        let D = is_key_down(KeyCode::D) || is_key_down(KeyCode::E);

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
                padding + f32::from(line) * line_height,
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
                padding + f32::from(line) * line_height,
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
                        padding + f32::from(line) * line_height,
                        font_size,
                        color,
                    );
                    line += 1;
                }
            }
        }

        if is_key_down(KeyCode::LeftControl) {
            top_down_camera_controls(&mut self.main_camera);
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

    pub fn draw(&self) {
        // Camera space, render game objects
        let zoom = vec2(self.main_camera.zoom.x, -self.main_camera.zoom.y);
        set_camera(&Camera2D {
            target: self.main_camera.target,
            rotation: -self.main_camera.rotation.to_degrees(),
            zoom,
            ..Camera2D::default()
        });

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

        set_default_camera();
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct Time {
    delta: f64,
    overall: f64,
}
