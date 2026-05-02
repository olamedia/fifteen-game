mod board;
mod consts;
mod game;
mod render;
mod scenes;
mod types;
mod ui;

use macroquad::prelude::*;
use macroquad::audio::load_sound_from_bytes;
use macroquad::miniquad::conf::Icon;

use crate::consts::*;
use crate::game::{Game, Sounds};

const FONT_BYTES: &[u8] = include_bytes!("../assets/Exo2-SemiBold.ttf");
const ICON16: &[u8; 1024] = include_bytes!("../assets/icon16.rgba");
const ICON32: &[u8; 4096] = include_bytes!("../assets/icon32.rgba");
const ICON64: &[u8; 16384] = include_bytes!("../assets/icon64.rgba");
const SFX_SLIDE: &[u8] = include_bytes!("../assets/slide.ogg");
const SFX_HIT: &[u8] = include_bytes!("../assets/hit.ogg");
const SFX_NAV: &[u8] = include_bytes!("../assets/navigate.ogg");
const SFX_SEL: &[u8] = include_bytes!("../assets/select.ogg");
const SFX_WIN: &[u8] = include_bytes!("../assets/win.ogg");

fn window_conf() -> Conf {
    Conf {
        window_title: "Fifteen Puzzle".to_string(),
        window_width: WINDOW_W,
        window_height: WINDOW_H,
        window_resizable: true,
        high_dpi: true,
        icon: Some(Icon {
            small: *ICON16,
            medium: *ICON32,
            big: *ICON64,
        }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font_from_bytes(FONT_BYTES).expect("Failed to load font");

    let sfx = Sounds {
        slide: load_sound_from_bytes(SFX_SLIDE).await.expect("slide.ogg"),
        hit: load_sound_from_bytes(SFX_HIT).await.expect("hit.ogg"),
        navigate: load_sound_from_bytes(SFX_NAV).await.expect("navigate.ogg"),
        select: load_sound_from_bytes(SFX_SEL).await.expect("select.ogg"),
        win: load_sound_from_bytes(SFX_WIN).await.expect("win.ogg"),
    };

    let mut game = Game::new(font, sfx);

    loop {
        let dt = get_frame_time().min(0.05);
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
