pub mod debug;
pub mod edit;
pub mod play;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use macroquad::prelude::*;

use crate::audio::Audio;
use crate::camera::{top_down_camera_controls, Camera};
use crate::collider::Collider;
use crate::entity::Entities;
use crate::player::Player;
use crate::sprite::Sprites;
use crate::static_layers::StaticEntity;
use crate::static_layers::StaticLayers;

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

    entities: Entities,

    // Play
    player: Option<Player>,
    // Edit
    chosen_entity: Option<usize>,
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

            entities: Entities::new(),
            player: None,
            chosen_entity: None,
        }
    }

    pub fn setup(&mut self) {
        self.load_level();
        self.entities.load_entities();
        let player_collider = &self
            .entities
            .find("char1-idle")
            .expect("Couldn't find player in entities")
            .collider;
        let sprites: Vec<String> = vec![
            "char1-idle".to_owned(),
            "char1-jump".to_owned(),
            "char1-fall".to_owned(),
        ];
        self.player = Some(Player::new(
            Vec2::new(0.0, 0.0),
            player_collider.clone(),
            &sprites,
        ));
    }

    pub fn input(&mut self) {
        match self.state {
            WorldState::Menu => (),
            WorldState::Play => self.play_input(),
            WorldState::Edit => self.edit_input(),
            WorldState::Debug => self.debug_input(),
        }

        if is_key_down(KeyCode::LeftControl) {
            top_down_camera_controls(&mut self.main_camera);
            if is_key_pressed(KeyCode::Space) {
                self.state = match self.state {
                    WorldState::Play => WorldState::Debug,
                    WorldState::Debug => WorldState::Edit,
                    WorldState::Edit => WorldState::Play,
                    _ => todo!(),
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.update_time(get_time());
        let delta = self.time.delta;

        match self.state {
            WorldState::Menu => (),
            WorldState::Play => self.play_update(delta),
            WorldState::Edit => self.edit_update(),
            WorldState::Debug => self.debug_update(),
        }

        self.main_camera.update();
    }

    fn update_time(&mut self, time: f64) {
        let speed = 1.0;
        self.time = Time {
            delta: (time - self.time.overall) * speed,
            overall: get_time(),
        };
    }

    pub fn draw(&mut self) {
        self.camera_set();
        match self.state {
            WorldState::Debug => self.debug_draw(),
            WorldState::Play => self.play_draw(),
            WorldState::Edit => self.edit_draw(),
            _ => todo!(),
        }
    }

    pub fn camera_set(&self) {
        // Camera space, render game objects
        let zoom = vec2(self.main_camera.zoom.x, -self.main_camera.zoom.y);
        set_camera(&Camera2D {
            target: self.main_camera.target,
            rotation: -self.main_camera.rotation.to_degrees(),
            zoom,
            ..Camera2D::default()
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_level(&self) {
        let path = Path::new("./data/level0.txt");
        let display = path.display();
        // Open a file in write-only mode, returns `io::Result<File>`
        match fs::read_dir("./data/") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => {
                for path in paths {
                    println!("> {:?}", path.unwrap().path());
                }
            }
        }

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(format!("{}", self.static_layers).as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn save_level(&self) {}

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_level(&mut self) {
        let path = Path::new("./data/level0.txt");
        let display = path.display();
        // Open a file in write-only mode, returns `io::Result<File>`

        match fs::read_dir("./data/") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => {
                for path in paths {
                    println!("> {:?}", path.unwrap().path());
                }
            }
        }

        let contents = fs::read_to_string(&path).expect("couldn't read the level");

        for line in contents.lines() {
            if line.is_empty() {
                continue;
            }
            let (token, rest) = line
                .split_once(' ')
                .expect("Every line has at least 1 space");
            match token {
                "StaticLayer" => {
                    println!("Parsed beginning of the new StaticLayer")
                }
                "StaticEntity" => {
                    let mut stuff = rest
                        .split_ascii_whitespace()
                        .filter(|e| e != &"{" && e != &"}");
                    let sprite = stuff.next().unwrap().trim_matches(',');
                    let pos_x = stuff
                        .next()
                        .unwrap()
                        .trim_matches(',')
                        .split(':')
                        .nth(1)
                        .expect("Didn't find a second item of y:f32 (<-- this)")
                        .parse::<f32>()
                        .expect("Could not parse the f32");

                    let pos_y = stuff
                        .next()
                        .unwrap()
                        .trim_matches(',')
                        .split(':')
                        .nth(1)
                        .expect("Didn't find a second item of y:f32 (<-- this)")
                        .parse::<f32>()
                        .expect("Could not parse the f32");

                    let mut collider = [0f32; 4];
                    for (pair, to_parse) in stuff.skip(1).enumerate() {
                        collider[pair] = to_parse
                            .trim_matches(',')
                            .split(':')
                            .nth(1)
                            .expect("Didn't find a second item of x:f32 (<-- this)")
                            .parse::<f32>()
                            .expect("Could not parse the f32");
                    }
                    let entity = StaticEntity::new(
                        Vec2::new(pos_x, pos_y),
                        sprite.to_owned(),
                        Collider::from(collider),
                    );
                    println!("parsed {}", entity);
                    self.static_layers.add_entity(0, entity);
                }
                _ => (),
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn load_level(&self) {}
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct Time {
    delta: f64,
    overall: f64,
}
