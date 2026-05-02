use macroquad::prelude::*;
use macroquad::audio::{Sound, play_sound, PlaySoundParams};

use crate::board::Board;
use crate::consts::*;
use crate::types::GameState;

pub struct Sounds {
    pub slide: Sound,
    pub hit: Sound,
    pub navigate: Sound,
    pub select: Sound,
    pub win: Sound,
}

pub struct TileAnim {
    pub from_row: usize,
    pub from_col: usize,
    pub to_row: usize,
    pub to_col: usize,
    pub t: f32,
}

pub struct Game {
    pub state: GameState,
    pub board: Board,
    pub size: usize,
    pub moves: u32,
    pub timer: f64,
    pub best_time: f64,
    pub best_moves: u32,
    pub menu_sel: usize,
    pub options_row: usize,
    pub state_timer: f32,
    pub volume: u8,
    pub font: Font,
    pub sfx: Sounds,
    pub anims: Vec<TileAnim>,
    pub pending_win: bool,
}

impl Game {
    pub fn new(font: Font, sfx: Sounds) -> Self {
        Self {
            state: GameState::Menu,
            board: Board::solved(DEFAULT_SIZE),
            size: DEFAULT_SIZE,
            moves: 0,
            timer: 0.0,
            best_time: 0.0,
            best_moves: 0,
            menu_sel: 0,
            options_row: 0,
            state_timer: 0.0,
            volume: 10,
            font,
            sfx,
            anims: vec![],
            pending_win: false,
        }
    }

    pub fn vol(&self) -> f32 {
        self.volume as f32 / 100.0
    }

    pub fn play(&self, s: &Sound) {
        play_sound(s, PlaySoundParams {
            volume: self.vol(),
            ..Default::default()
        });
    }

    pub fn start_game(&mut self) {
        self.board = Board::new(self.size);
        self.moves = 0;
        self.timer = 0.0;
        self.anims.clear();
        self.pending_win = false;
        self.state = GameState::Playing;
        self.state_timer = 0.0;
    }

    pub fn cell_size(&self) -> f32 {
        let max_board = screen_width().min(screen_height() - 120.0) - 60.0;
        (max_board / self.size as f32).floor()
    }

    pub fn board_origin(&self) -> (f32, f32) {
        let cs = self.cell_size();
        let total = self.size as f32 * cs;
        let ox = (screen_width() - total) / 2.0;
        let oy = (screen_height() - total) / 2.0 + 30.0;
        (ox, oy)
    }

    pub fn text(&self, txt: &str, x: f32, y: f32, size: f32, color: Color) {
        draw_text_ex(txt, x, y, TextParams {
            font: Some(&self.font),
            font_size: size as u16,
            font_scale: 1.0,
            color,
            ..Default::default()
        });
    }

    pub fn measure(&self, txt: &str, size: f32) -> TextDimensions {
        measure_text(txt, Some(&self.font), size as u16, 1.0)
    }

    pub fn update(&mut self, dt: f32) {
        self.state_timer += dt;
        match self.state {
            GameState::Menu => self.update_menu(),
            GameState::Options => self.update_options(),
            GameState::Playing => self.update_playing(dt),
            GameState::Victory => self.update_victory(),
        }
    }

    pub fn draw(&mut self) {
        clear_background(BG);
        match self.state {
            GameState::Menu => self.draw_menu(),
            GameState::Options => self.draw_options(),
            GameState::Playing => self.draw_playing(),
            GameState::Victory => self.draw_victory(),
        }
    }

    pub fn format_time(secs: f64) -> String {
        let m = (secs / 60.0) as u32;
        let s = (secs % 60.0) as u32;
        format!("{:02}:{:02}", m, s)
    }
}
