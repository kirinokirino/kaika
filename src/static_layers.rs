use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};

use macroquad::math::Vec2;
use macroquad::prelude::Rect;

use crate::collider::Collider;
use crate::common::Direction;
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
    ) -> (Option<Direction>, Vec2) {
        if let Some(layer) = self.layer.get(&layer) {
            let target = end_position - start_position;
            let direction = target.normalize();
            debug_assert!(!(direction.x.is_nan() && direction.y.is_nan()));
            let length = target.length();

            let colliders: Vec<Rect> = layer
                .iter()
                .filter(|collider| collider.collider.is_some())
                .map(|entity| {
                    entity
                        .collider
                        .clone()
                        .expect("the one without a collider should be filtered out")
                        .rect()
                        .offset(entity.pos)
                })
                .collect();
            for step in 0..length.floor() as i32 {
                let step_offset_x = Vec2::new(step as f32 * direction.x, 0.0) + start_position;
                let temp_collider_x = collider.rect().offset(step_offset_x);
                let step_offset_y = Vec2::new(0.0, step as f32 * direction.y) + start_position;
                let temp_collider_y = collider.rect().offset(step_offset_y);
                for check_against in &colliders {
                    if let Some(collision_rect) = check_against.intersect(temp_collider_x) {
                        if direction.x > 0.0 {
                            return (Some(Direction::Right), step_offset_x);
                        } else {
                            return (Some(Direction::Left), step_offset_x);
                        }
                    } else if let Some(collision_rect) = check_against.intersect(temp_collider_y) {
                        if direction.y > 0.0 {
                            return (Some(Direction::Bottom), step_offset_y);
                        } else {
                            return (Some(Direction::Top), step_offset_y);
                        }
                    }
                }
            }
        }
        (None, end_position)
    }

    pub fn replace(&mut self, entity: StaticEntity) {
        let z_index = 0;
        let is_new_layer = self.used_layers.insert(z_index);
        if is_new_layer {
            println!("Creating a new z_index layer {}", z_index);
            let new_layer: Vec<StaticEntity> = vec![entity];
            self.layer.insert(z_index, new_layer);
        } else {
            for layer_entity in self
                .layer
                .get_mut(&z_index)
                .expect("Layer found in used_layers did not actually exist.")
            {
                if layer_entity.sprite == entity.sprite {
                    *layer_entity = entity;
                    return;
                }
            }
        }
    }

    #[allow(clippy::unwrap_in_result)]
    pub fn find(&self, name: &str) -> Option<&StaticEntity> {
        for layer in &self.used_layers {
            let layer = self
                .layer
                .get(layer)
                .expect("Unexisting layer in StaticLayers::used_layers");
            for entity in layer {
                if entity.sprite == name {
                    return Some(entity);
                }
            }
        }
        None
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
    pub collider: Option<Collider>,
    pub sprite: String,
}

impl StaticEntity {
    #[must_use]
    pub fn new(pos: Vec2, sprite: String, collider: Collider) -> Self {
        if collider.is_zero() {
            Self {
                pos,
                collider: None,
                sprite,
            }
        } else {
            Self {
                pos,
                collider: Some(collider),
                sprite,
            }
        }
    }

    pub fn debug(&self, sprites: &Sprites) {
        sprites.draw(&self.sprite, self.pos);
        if let Some(collider) = &self.collider {
            collider.draw(self.pos);
        }
    }
}

impl Display for StaticEntity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(collider) = &self.collider {
            write!(
                f,
                "StaticEntity {{ {}, x:{}, y:{}, {} }}",
                self.sprite, self.pos.x, self.pos.y, collider
            )
        } else {
            write!(
                f,
                "StaticEntity {{ {}, x:{}, y:{} }}",
                self.sprite, self.pos.x, self.pos.y
            )
        }
    }
}
