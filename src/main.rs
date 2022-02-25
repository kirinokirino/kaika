#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::unwrap_in_result,
    clippy::unneeded_field_pattern,
    clippy::string_to_string,
    clippy::string_slice,
    clippy::string_add,
    clippy::str_to_string,
    clippy::same_name_method,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::rc_mutex,
    clippy::rc_buffer,
    clippy::pattern_type_mismatch,
    clippy::missing_enforced_import_renames,
    clippy::lossy_float_literal,
    clippy::let_underscore_must_use,
    clippy::integer_division,
    clippy::inline_asm_x86_att_syntax,
    clippy::indexing_slicing,
    clippy::if_then_some_else_none,
    clippy::get_unwrap,
    clippy::fn_to_numeric_cast,
    clippy::float_cmp_const,
    clippy::filetype_is_file,
    clippy::create_dir,
    clippy::clone_on_ref_ptr,
    clippy::as_conversions,
    clippy::verbose_file_reads
)]
#![allow(
    clippy::cast_precision_loss,
    clippy::missing_panics_doc,
    clippy::option_if_let_else,
    clippy::pattern_type_mismatch
)]

use macroquad::prelude::*;

mod audio;
mod camera;
mod collider;
mod common;
mod entity;
mod player;
mod sprite;
mod static_layers;
mod tween;
mod world;

pub const SPRITE_PATHS: &str = include_str!("../data/sprites.txt");
pub const SOUND_PATHS: &str = include_str!("../data/sounds.txt");

fn window_conf() -> Conf {
    Conf {
        window_title: "game".to_owned(),
        fullscreen: false,
        ..macroquad::window::Conf::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let audio = audio::Audio::new().await;
    let sprites = sprite::Sprites::new().await;
    let mut world = world::World::new(audio, sprites);

    world.setup();

    loop {
        clear_background(color_u8!(35, 47, 54, 255));

        world.input();
        world.update();
        world.draw();

        next_frame().await;
    }
}
