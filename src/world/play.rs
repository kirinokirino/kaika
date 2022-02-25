use macroquad::prelude::*;

use crate::world::World;

impl World {
    pub fn play_input(&mut self) {
        let _w = is_key_down(KeyCode::W) || is_key_down(KeyCode::Comma);
        let _s = is_key_down(KeyCode::S) || is_key_down(KeyCode::O);
        let a = is_key_down(KeyCode::A);
        let d = is_key_down(KeyCode::D) || is_key_down(KeyCode::E);

        if let Some(player) = &mut self.player {
            if is_key_down(KeyCode::Space) {
                player.jump();
            } else {
                player.jump_stop();
            }
            if a {
                player.left();
            } else if d {
                player.right();
            } else {
                player.stop();
            }
        }
    }

    pub fn play_update(&mut self, delta: f64) {
        if let Some(player) = &mut self.player {
            player.update(delta);
            self.main_camera.target += (player.pos - self.main_camera.target) * 0.2;
        }
    }

    pub fn play_draw(&self) {
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

        if let Some(player) = &self.player {
            player.draw(&self.sprites);
            player.collider.draw(player.pos);
        }

        set_default_camera();

        if let Some(player) = &self.player {
            draw_text(
                &format!("{}", player),
                10.0,
                10.0,
                18.0,
                color_u8!(255, 255, 255, 255),
            );
        }
        draw_text(
            &format!("FPS: {}", get_fps()),
            10.0,
            30.0,
            18.0,
            color_u8!(255, 255, 255, 255),
        );
    }
}
