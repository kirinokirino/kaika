use std::collections::HashMap;

use macroquad::audio::Sound;
use macroquad::ui;

use crate::SOUND_PATHS;

pub struct Audio {
    samples: HashMap<&'static str, Sample>,
}

impl Audio {
    pub async fn new() -> Self {
        let paths = SOUND_PATHS.split('\n');
        let mut samples = HashMap::with_capacity(5);
        for path in paths {
            let name = path
                .split('/')
                .last()
                .expect("Path should be in a samples/ folder, thus have a '/'")
                .split('.')
                .take(1)
                .last()
                .expect("Path should include .ogg extension");
            let sample = Sample::new(path).await;
            samples.insert(name, sample);
        }
        Self { samples }
    }

    pub fn debug(&self) {
        for (name, sample) in &self.samples {
            if ui::root_ui().button(None, *name) {
                macroquad::audio::play_sound_once(sample.sound);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Sample {
    path: &'static str,
    sound: Sound,
}

impl Sample {
    pub async fn new(path: &'static str) -> Self {
        let sound = macroquad::audio::load_sound(path)
            .await
            .expect("Sound file loading error");
        Self { path, sound }
    }
}
