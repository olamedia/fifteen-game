use macroquad::prelude::*;

use crate::consts::*;
use crate::game::Game;
use crate::types::GameState;
use crate::ui::{ClickableText, apply_cursor};

const VIC_ITEMS: [&str; 2] = ["PLAY AGAIN", "MENU"];
const PAD: f32 = 30.0;
const PANEL_W: f32 = 340.0;

struct VicLayout {
    top: f32,
    panel_h: f32,
    title_y: f32,
    moves_y: f32,
    time_y: f32,
    best_y: f32,
    has_best: bool,
    items_y: [f32; 2],
}

impl Game {
    fn vic_layout(&self) -> VicLayout {
        let cy = screen_height() / 2.0;
        let th = self.measure("X", 48.0).height;

        let mut h: f32 = 0.0;
        let title_y = h + th;
        h += th + 36.0;

        let moves_y = h;
        h += 30.0;
        let time_y = h;
        h += 30.0;

        let has_best = self.best_moves > 0;
        let best_y = h;
        if has_best {
            h += 28.0;
        }

        h += 14.0;
        let mut items_y = [0.0_f32; 2];
        for i in 0..2 {
            items_y[i] = h;
            h += 36.0;
        }
        h -= 10.0;

        let panel_h = PAD * 2.0 + h;
        let top = cy - panel_h / 2.0;
        let off = top + PAD;

        VicLayout {
            top,
            panel_h,
            title_y: off + title_y,
            moves_y: off + moves_y,
            time_y: off + time_y,
            best_y: off + best_y,
            has_best,
            items_y: [off + items_y[0], off + items_y[1]],
        }
    }

    pub fn update_victory(&mut self) {
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            if self.menu_sel > 0 {
                self.menu_sel -= 1;
                self.play(&self.sfx.navigate.clone());
            }
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            if self.menu_sel < VIC_ITEMS.len() - 1 {
                self.menu_sel += 1;
                self.play(&self.sfx.navigate.clone());
            }
        }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            self.activate_victory();
        }
        if is_key_pressed(KeyCode::Escape) {
            self.play(&self.sfx.select.clone());
            self.state = GameState::Menu;
            self.state_timer = 0.0;
        }
    }

    fn activate_victory(&mut self) {
        self.play(&self.sfx.select.clone());
        match self.menu_sel {
            0 => self.start_game(),
            1 => {
                self.state = GameState::Menu;
                self.state_timer = 0.0;
            }
            _ => {}
        }
    }

    pub fn draw_victory(&mut self) {
        let cx = screen_width() / 2.0;
        let mut hovering = false;

        let (ox, oy) = self.board_origin();
        self.draw_board_bg(ox, oy);
        self.draw_all_tiles(ox, oy);

        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.3));

        let lay = self.vic_layout();
        self.draw_panel(cx, lay.top + lay.panel_h / 2.0, PANEL_W, lay.panel_h);

        let title = "SOLVED!";
        let tw = self.measure(title, 48.0).width;
        self.text(title, cx - tw / 2.0, lay.title_y, 48.0, ACCENT);

        let moves_str = format!("Moves: {}", self.moves);
        let mw = self.measure(&moves_str, 24.0).width;
        self.text(&moves_str, cx - mw / 2.0, lay.moves_y, 24.0, TEXT_DARK);

        let time_str = format!("Time: {}", Game::format_time(self.timer));
        let tw2 = self.measure(&time_str, 24.0).width;
        self.text(&time_str, cx - tw2 / 2.0, lay.time_y, 24.0, TEXT_DARK);

        if lay.has_best {
            let best = format!("Best: {} / {}", self.best_moves, Game::format_time(self.best_time));
            let bw = self.measure(&best, 16.0).width;
            self.text(&best, cx - bw / 2.0, lay.best_y, 16.0, TEXT_WARM);
        }

        for i in 0..VIC_ITEMS.len() {
            let y = lay.items_y[i];
            let size = if self.menu_sel == i { 26.0 } else { 22.0 };
            let color = if self.menu_sel == i { ACCENT } else { TEXT_DIM };

            let btn = ClickableText::new(VIC_ITEMS[i], cx, y, size, color, ACCENT, &self.font);
            btn.draw();
            if btn.hovered() {
                hovering = true;
            }
            if btn.clicked() {
                self.menu_sel = i;
                self.activate_victory();
            }
        }

        apply_cursor(hovering);
    }
}
