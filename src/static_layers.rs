/// collider
/// sprite
use std::collections::{HashMap, HashSet};

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
}

pub struct StaticEntity {
    collider: Collider,
    sprite: &'static str,
}

impl StaticEntity {
    #[must_use]
    pub const fn new(sprite: &'static str, collider: Collider) -> Self {
        Self { collider, sprite }
    }

    pub fn debug(&self, sprites: &Sprites) {
        self.collider.draw();
        sprites.draw(self.sprite, self.collider.pos);
    }
}
