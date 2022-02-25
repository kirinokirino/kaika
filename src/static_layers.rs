use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};

use macroquad::math::Vec2;

use crate::collider::Collider;
use crate::sprite::Sprites;

// by z-index, I guess
pub struct StaticLayers {
    layer: HashMap<i32, Vec<StaticEntity>>,
    used_layers: HashSet<i32>,
}

impl StaticLayers {
    pub fn new() -> Self {
        Self {
            layer: HashMap::new(),
            used_layers: HashSet::new(),
        }
    }

    pub fn add_entity(&mut self, z_index: i32, entity: StaticEntity) {
        let is_new_layer = self.used_layers.insert(z_index);
        if is_new_layer {
            println!("Creating a new z_index layer {}", z_index);
            let new_layer: Vec<StaticEntity> = vec![entity];
            self.layer.insert(z_index, new_layer);
        } else {
            self.layer
                .get_mut(&z_index)
                .expect("Layer found in used_layers did not actually exist.")
                .push(entity);
        }
    }

    pub fn draw(&self, sprites: &Sprites) {
        for layer in &self.used_layers {
            let layer = self
                .layer
                .get(layer)
                .expect("Unexisting layer in StaticLayers::used_layers");
            for entity in layer {
                entity.debug(sprites);
            }
        }
    }

    pub fn get_collision_point(
        &self,
        collider: &Collider,
        start_position: Vec2,
        end_position: Vec2,
        layer: i32,
    ) -> Vec2 {
        let target = end_position - start_position;
        let direction = target.normalize();
        let length = target.length();
        if length < 1.0 {
            let temp_collider = collider.rect().offset(target + start_position);
            if let Some(layer) = self.layer.get(&layer) {
                for collider in layer {
                    if let Some(collision_rect) = collider
                        .collider
                        .rect()
                        .offset(collider.pos)
                        .intersect(temp_collider)
                    {
                        return start_position;
                    }
                }
            }
            return end_position;
        }
        for step in 1..length.floor() as i32 {
            let temp_collider = collider
                .rect()
                .offset(step as f32 * direction + start_position);

            if let Some(layer) = self.layer.get(&layer) {
                for collider in layer {
                    if let Some(collision_rect) = collider
                        .collider
                        .rect()
                        .offset(collider.pos)
                        .intersect(temp_collider)
                    {
                        return step as f32 * direction + start_position;
                    }
                }
            }
        }
        end_position
    }
}

impl Display for StaticLayers {
    #[allow(clippy::unwrap_in_result)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut contents = String::new();
        for layer in &self.used_layers {
            let mut layer_str: String = format!("StaticLayer {}:\n", layer);
            let layer = self
                .layer
                .get(layer)
                .expect("Unexisting layer in StaticLayers::used_layers");
            for entity in layer {
                layer_str.push_str(&format!("{}\n", entity));
            }
            contents.push_str(&format!("{}\n", &layer_str));
        }
        write!(f, "{}", contents)
    }
}

#[derive(Clone)]
pub struct StaticEntity {
    pub pos: Vec2,
    pub collider: Collider,
    pub sprite: String,
}

impl StaticEntity {
    #[must_use]
    pub const fn new(pos: Vec2, sprite: String, collider: Collider) -> Self {
        Self {
            pos,
            collider,
            sprite,
        }
    }

    pub fn debug(&self, sprites: &Sprites) {
        sprites.draw(&self.sprite, self.pos);
        self.collider.draw(self.pos);
    }
}

impl Display for StaticEntity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StaticEntity {{ {}, x:{}, y:{}, {} }}",
            self.sprite, self.pos.x, self.pos.y, self.collider
        )
    }
}
