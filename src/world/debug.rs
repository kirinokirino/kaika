use macroquad::prelude::*;

use crate::world::World;

impl World {
    pub fn debug_input(&mut self) {
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

    pub fn debug_update(&mut self) {}

    pub fn debug_draw(&mut self) {
        set_default_camera();
        self.audio.debug();
        self.sprites.debug();
    }
}
