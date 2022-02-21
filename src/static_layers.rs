/// collider
/// sprite
use crate::collider::Collider;
use crate::sprite::Sprites;

// by z-index, I guess
struct StaticLayers {
    layer: Vec<Vec<StaticEntity>>,
}

struct StaticEntity {
    collider: Collider,
    sprite: String,
}

impl StaticEntity {
    pub fn new(sprite: String, collider: Collider) -> Self {
        Self { collider, sprite }
    }

    pub fn debug(&self, sprites: &Sprites) {
        self.collider.draw();
        sprites.draw(&self.sprite, self.collider.pos);
    }
}
