use macroquad::prelude::Color;

pub const WINDOW_W: i32 = 700;
pub const WINDOW_H: i32 = 750;

pub const DEFAULT_SIZE: usize = 4;
pub const MIN_SIZE: usize = 3;
pub const MAX_SIZE: usize = 6;

pub const TILE_ROUND: f32 = 8.0;
pub const TILE_GAP: f32 = 4.0;
pub const ANIM_SPEED: f32 = 12.0;

pub const BG: Color = Color::new(0.96, 0.94, 0.90, 1.0);
pub const BOARD_BG: Color = Color::new(0.88, 0.85, 0.80, 1.0);
pub const TILE_COLOR: Color = Color::new(0.98, 0.96, 0.92, 1.0);
pub const TILE_SHADOW: Color = Color::new(0.0, 0.0, 0.0, 0.10);
pub const TILE_TEXT: Color = Color::new(0.30, 0.25, 0.20, 1.0);
pub const TILE_CORRECT: Color = Color::new(0.55, 0.78, 0.55, 0.25);
pub const TEXT_DARK: Color = Color::new(0.25, 0.22, 0.18, 1.0);
pub const TEXT_WARM: Color = Color::new(0.72, 0.50, 0.25, 1.0);
pub const TEXT_DIM: Color = Color::new(0.55, 0.52, 0.48, 1.0);
pub const ACCENT: Color = Color::new(0.82, 0.52, 0.20, 1.0);
pub const ACCENT_DIM: Color = Color::new(0.72, 0.50, 0.25, 0.6);
