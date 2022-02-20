use macroquad::prelude::*;

mod audio;
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

    let audio = audio::Audio::new().await;

    loop {
        clear_background(GRAY);

        audio.debug();

        next_frame().await
    }
}
