use macroquad::prelude::*;

use crate::consts::*;
use crate::game::Game;
use crate::types::GameState;
use crate::ui::{ClickableText, apply_cursor};

const MENU_ITEMS: [&str; 3] = ["PLAY", "OPTIONS", "EXIT"];

impl Game {
    pub fn update_menu(&mut self) {
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            if self.menu_sel > 0 {
                self.menu_sel -= 1;
                self.play(&self.sfx.navigate.clone());
            }
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            if self.menu_sel < MENU_ITEMS.len() - 1 {
                self.menu_sel += 1;
                self.play(&self.sfx.navigate.clone());
            }
        }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            self.activate_menu();
        }
    }

    fn activate_menu(&mut self) {
        self.play(&self.sfx.select.clone());
        match self.menu_sel {
            0 => self.start_game(),
            1 => {
                self.state = GameState::Options;
                self.state_timer = 0.0;
            }
            2 => std::process::exit(0),
            _ => {}
        }
    }

    pub fn draw_menu(&mut self) {
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        let mut hovering = false;

        let title = "FIFTEEN";
        let ts = 64.0;
        let tw = self.measure(title, ts).width;
        self.text(title, cx - tw / 2.0, cy - 100.0, ts, TEXT_WARM);

        let sub = "slide puzzle";
        let ss = 20.0;
        let sw = self.measure(sub, ss).width;
        self.text(sub, cx - sw / 2.0, cy - 65.0, ss, TEXT_DIM);

        let line_y = cy - 50.0;
        let line_w = 120.0;
        draw_line(cx - line_w, line_y, cx + line_w, line_y, 1.0, ACCENT_DIM);

        for (i, item) in MENU_ITEMS.iter().enumerate() {
            let y = cy + i as f32 * 52.0;
            let size = if self.menu_sel == i { 34.0 } else { 28.0 };
            let color = if self.menu_sel == i { ACCENT } else { TEXT_DIM };

            let btn = ClickableText::new(item, cx, y, size, color, ACCENT, &self.font);
            btn.draw();
            if btn.hovered() {
                hovering = true;
            }
            if btn.clicked() {
                self.menu_sel = i;
                self.activate_menu();
            }

            if self.menu_sel == i {
                let iw = self.measure(item, size).width;
                let aw = self.measure(">", size).width;
                self.text(">", cx - iw / 2.0 - aw - 12.0, y, size, color);
            }
        }

        let hint = "ARROWS / WASD  NAVIGATE    ENTER  SELECT";
        let hw = self.measure(hint, 14.0).width;
        self.text(hint, cx - hw / 2.0, screen_height() - 30.0, 14.0, TEXT_DIM);

        apply_cursor(hovering);
    }
}
