use macroquad::prelude::*;
use macroquad::{audio, ui};

mod world;

fn window_conf() -> Conf {
    Conf {
        window_title: "game".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let sound1 = audio::load_sound("samples/unfa-ui-open.ogg").await.unwrap();
    let sound2 = audio::load_sound("samples/unfa-ui-close.ogg")
        .await
        .unwrap();
    let sound3 = audio::load_sound("samples/unfa-ui-hide.ogg").await.unwrap();
    let sound4 = audio::load_sound("samples/unfa-ui-select.ogg")
        .await
        .unwrap();
    let sound5 = audio::load_sound("samples/unfa-radar.ogg").await.unwrap();

    loop {
        clear_background(GRAY);

        if ui::root_ui().button(None, "Play sound 1") {
            warn!("play 1!");
            audio::play_sound_once(sound1);
        }
        if ui::root_ui().button(None, "Play sound 2") {
            warn!("play 2!");
            audio::play_sound_once(sound2);
        }
        if ui::root_ui().button(None, "Play sound 3") {
            warn!("play 3!");
            audio::play_sound_once(sound3);
        }
        if ui::root_ui().button(None, "Play sound 4") {
            warn!("play 4!");
            audio::play_sound_once(sound4);
        }
        if ui::root_ui().button(None, "Play sound 5") {
            warn!("play 5!");
            audio::play_sound_once(sound5);
        }

        next_frame().await
    }
}
