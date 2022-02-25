use std::fs;
use std::path::Path;

use macroquad::camera::set_default_camera;
use macroquad::math::Vec2;
use macroquad::ui;

use crate::collider::Collider;
use crate::static_layers::StaticEntity;

pub struct Entities {
    static_entities: Vec<StaticEntity>,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            static_entities: Vec::new(),
        }
    }

    pub fn get(&self, entity: usize) -> Option<&StaticEntity> {
        self.static_entities.get(entity)
    }

    pub fn find(&self, sprite_name: &str) -> Option<&StaticEntity> {
        for entity in &self.static_entities {
            if entity.sprite == sprite_name {
                return Some(entity);
            }
        }
        return None;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_entities(&mut self) {
        let path = Path::new("./data/entities.txt");
        let display = path.display();
        println!("Loading entities from {}", display);
        self.static_entities.clear();
        // Open a file in write-only mode, returns `io::Result<File>`

        let contents = fs::read_to_string(&path).expect("couldn't read the level");
        for line in contents.lines() {
            println!("Parsing {}", line);
            let (token, rest) = line
                .split_once(' ')
                .expect("Every line has at least 1 space");
            match token {
                "Entity" => {
                    println!("Parsed beginning of entity")
                }
                "StaticEntity" => {
                    let stuff = rest
                        .split_ascii_whitespace()
                        .filter(|e| e != &"{" && e != &"}");
                    let sprite = stuff.clone().take(1).last().unwrap().trim_matches(',');

                    let mut collider = [0f32; 4];
                    for (pair, to_parse) in stuff.skip(2).enumerate() {
                        collider[pair] = to_parse
                            .trim_matches(',')
                            .split(':')
                            .nth(1)
                            .expect("Didn't find a second item of x:f32 (<-- this)")
                            .parse::<f32>()
                            .expect("Could not parse the f32");
                    }
                    let entity = StaticEntity::new(
                        Vec2::new(0.0, 0.0),
                        sprite.to_owned(),
                        Collider::from(collider),
                    );
                    println!("parsed {}", entity);
                    self.static_entities.push(entity);
                }
                _ => (),
            }
        }
    }
    pub fn ui(&self) -> Option<usize> {
        set_default_camera();
        let mut selection = None;
        for (i, entity) in self.static_entities.iter().enumerate() {
            if ui::root_ui().button(None, entity.sprite.clone()) {
                selection = Some(i);
            }
        }
        selection
    }
}

struct Entity {}

impl Entity {}
