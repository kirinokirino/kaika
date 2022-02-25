use macroquad::prelude::*;
use macroquad::ui;

use crate::world::World;

impl World {
    pub fn edit_input(&mut self) {
        if is_key_pressed(KeyCode::Key1) {
            self.entities.load_entities();
        }

        let mouse = self.main_camera.mouse_world_position();

        let lmb = is_mouse_button_pressed(MouseButton::Left);

        if let Some(entity) = self.chosen_entity {
            let entity = self
                .entities
                .get(entity)
                .expect("Tried to get unexisting entity, chosen_entity set incorrectly");
            if lmb {
                let mut entity = entity.clone();
                let offset = if let Some(collider) = &entity.collider {
                    collider.center()
                } else {
                    self.sprites
                        .get_sprite(&entity.sprite)
                        .expect("Chosen entity should have a sprite")
                        .size()
                        * 0.5
                };
                entity.pos = mouse - offset;
                self.static_layers.add_entity(0, entity);
            }
        }

        set_default_camera();
        if let Some(chosen) = self.entities.ui() {
            self.chosen_entity = Some(chosen);
        }

        if ui::root_ui().button(None, "Save the level") {
            self.save_level();
        }
    }
    pub fn edit_update(&mut self) {}
    pub fn edit_draw(&mut self) {
        self.static_layers.draw(&self.sprites);
        if let Some(entity) = self.chosen_entity {
            let mouse = self.main_camera.mouse_world_position();
            let entity = self
                .entities
                .get(entity)
                .expect("Tried to get unexisting entity, chosen_entity set incorrectly");

            self.sprites.draw_at_center(&entity.sprite, mouse);
        }
    }
}
