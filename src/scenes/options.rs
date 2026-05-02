use macroquad::prelude::*;

use crate::consts::*;
use crate::game::Game;
use crate::types::GameState;
use crate::ui::{ClickableText, apply_cursor};

const NUM_OPTIONS: usize = 2;

impl Game {
    pub fn update_options(&mut self) {
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            if self.options_row > 0 {
                self.options_row -= 1;
                self.play(&self.sfx.navigate.clone());
            }
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            if self.options_row < NUM_OPTIONS - 1 {
                self.options_row += 1;
                self.play(&self.sfx.navigate.clone());
            }
        }

        let left = is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A);
        let right = is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D);

        let mut changed = false;
        match self.options_row {
            0 => {
                if left && self.size > MIN_SIZE {
                    self.size -= 1;
                    changed = true;
                }
                if right && self.size < MAX_SIZE {
                    self.size += 1;
                    changed = true;
                }
            }
            1 => {
                if left && self.volume > 0 {
                    self.volume = self.volume.saturating_sub(10);
                    changed = true;
                }
                if right && self.volume < 100 {
                    self.volume = (self.volume + 10).min(100);
                    changed = true;
                }
            }
            _ => {}
        }
        if changed {
            self.play(&self.sfx.navigate.clone());
        }

        if is_key_pressed(KeyCode::Escape)
            || is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::Space)
        {
            self.play(&self.sfx.select.clone());
            self.state = GameState::Menu;
            self.state_timer = 0.0;
        }
    }

    pub fn draw_options(&mut self) {
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        let mut hovering = false;

        let title = "OPTIONS";
        let tw = self.measure(title, 48.0).width;
        self.text(title, cx - tw / 2.0, cy - 80.0, 48.0, TEXT_WARM);

        let row_ys = [cy, cy + 70.0];
        for (row, &ry) in row_ys.iter().enumerate() {
            let selected = self.options_row == row;
            let label = match row {
                0 => "SIZE",
                _ => "VOLUME",
            };
            let value = self.opt_row_text(row);
            let has_left = match row {
                0 => self.size > MIN_SIZE,
                _ => self.volume > 0,
            };
            let has_right = match row {
                0 => self.size < MAX_SIZE,
                _ => self.volume < 100,
            };

            let label_color = if selected { TEXT_DARK } else { TEXT_DIM };
            let lw = self.measure(label, 20.0).width;
            self.text(label, cx - lw / 2.0, ry, 20.0, label_color);

            let sel_size = if selected { 30.0 } else { 26.0 };
            let sel_color = if selected { ACCENT } else { ACCENT_DIM };

            let left_str = if has_left { "<" } else { " " };
            let right_str = if has_right { ">" } else { " " };

            let full = format!("{}  {}  {}", left_str, value, right_str);
            let fw = self.measure(&full, sel_size).width;
            let arrow_y = ry + 32.0;
            self.text(&full, cx - fw / 2.0, arrow_y, sel_size, sel_color);

            if has_left {
                let lx = cx - fw / 2.0;
                let btn = ClickableText::new_left("<", lx, arrow_y, sel_size, sel_color, ACCENT, &self.font);
                if btn.hovered() {
                    hovering = true;
                    if is_mouse_button_pressed(MouseButton::Left) {
                        self.options_row = row;
                        self.click_opt_left();
                    }
                }
            }
            if has_right {
                let arrow_w = self.measure(">", sel_size).width;
                let rx = cx + fw / 2.0 - arrow_w;
                let btn = ClickableText::new_left(">", rx, arrow_y, sel_size, sel_color, ACCENT, &self.font);
                if btn.hovered() {
                    hovering = true;
                    if is_mouse_button_pressed(MouseButton::Left) {
                        self.options_row = row;
                        self.click_opt_right();
                    }
                }
            }
        }

        let back = ClickableText::new("BACK", cx, cy + 180.0, 18.0, TEXT_DIM, ACCENT, &self.font);
        back.draw();
        if back.hovered() {
            hovering = true;
        }
        if back.clicked() {
            self.play(&self.sfx.select.clone());
            self.state = GameState::Menu;
            self.state_timer = 0.0;
        }

        apply_cursor(hovering);
    }

    fn opt_row_text(&self, row: usize) -> String {
        match row {
            0 => format!("{}x{}", self.size, self.size),
            1 => {
                if self.volume == 0 {
                    "MUTE".to_string()
                } else {
                    format!("{}%", self.volume)
                }
            }
            _ => String::new(),
        }
    }

    fn click_opt_left(&mut self) {
        match self.options_row {
            0 => {
                if self.size > MIN_SIZE {
                    self.size -= 1;
                    self.play(&self.sfx.navigate.clone());
                }
            }
            1 => {
                if self.volume > 0 {
                    self.volume = self.volume.saturating_sub(10);
                    self.play(&self.sfx.navigate.clone());
                }
            }
            _ => {}
        }
    }

    fn click_opt_right(&mut self) {
        match self.options_row {
            0 => {
                if self.size < MAX_SIZE {
                    self.size += 1;
                    self.play(&self.sfx.navigate.clone());
                }
            }
            1 => {
                if self.volume < 100 {
                    self.volume = (self.volume + 10).min(100);
                    self.play(&self.sfx.navigate.clone());
                }
            }
            _ => {}
        }
    }
}
