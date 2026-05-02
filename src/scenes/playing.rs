use macroquad::prelude::*;

use crate::consts::*;
use crate::game::{Game, TileAnim};
use crate::types::GameState;
use crate::ui::apply_cursor;

impl Game {
    pub fn update_playing(&mut self, dt: f32) {
        self.timer += dt as f64;

        let mut anim_done = false;
        for a in &mut self.anims {
            a.t += dt * ANIM_SPEED;
            if a.t < 1.0 {
                anim_done = false;
            }
        }
        if !self.anims.is_empty() && self.anims.iter().all(|a| a.t >= 1.0) {
            anim_done = true;
        }
        if anim_done {
            self.anims.clear();
            self.play(&self.sfx.hit.clone());
            if self.pending_win {
                self.pending_win = false;
                self.win();
                return;
            }
        }

        if !self.anims.is_empty() {
            return;
        }

        let mut move_count: u32 = 0;

        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            let old = self.board.empty_pos();
            if let Some((fr, fc)) = self.board.try_move_dir(-1, 0) {
                self.anims.push(TileAnim {
                    from_row: fr, from_col: fc,
                    to_row: old.0, to_col: old.1,
                    t: 0.0,
                });
                move_count = 1;
            }
        }
        if move_count == 0 && (is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S)) {
            let old = self.board.empty_pos();
            if let Some((fr, fc)) = self.board.try_move_dir(1, 0) {
                self.anims.push(TileAnim {
                    from_row: fr, from_col: fc,
                    to_row: old.0, to_col: old.1,
                    t: 0.0,
                });
                move_count = 1;
            }
        }
        if move_count == 0 && (is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A)) {
            let old = self.board.empty_pos();
            if let Some((fr, fc)) = self.board.try_move_dir(0, -1) {
                self.anims.push(TileAnim {
                    from_row: fr, from_col: fc,
                    to_row: old.0, to_col: old.1,
                    t: 0.0,
                });
                move_count = 1;
            }
        }
        if move_count == 0 && (is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D)) {
            let old = self.board.empty_pos();
            if let Some((fr, fc)) = self.board.try_move_dir(0, 1) {
                self.anims.push(TileAnim {
                    from_row: fr, from_col: fc,
                    to_row: old.0, to_col: old.1,
                    t: 0.0,
                });
                move_count = 1;
            }
        }

        if move_count == 0 && is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let (ox, oy) = self.board_origin();
            let cs = self.cell_size();
            let col = ((mx - ox) / cs) as i32;
            let row = ((my - oy) / cs) as i32;
            if row >= 0 && row < self.size as i32 && col >= 0 && col < self.size as i32 {
                let r = row as usize;
                let c = col as usize;
                let slides = self.board.try_slide(r, c);
                if !slides.is_empty() {
                    move_count = slides.len() as u32;
                    for (fr, fc, tr, tc) in slides {
                        self.anims.push(TileAnim {
                            from_row: fr, from_col: fc,
                            to_row: tr, to_col: tc,
                            t: 0.0,
                        });
                    }
                }
            }
        }

        if move_count > 0 {
            self.moves += move_count;
            self.play(&self.sfx.slide.clone());
            if self.board.is_solved() {
                self.pending_win = true;
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::Menu;
            self.state_timer = 0.0;
        }
    }

    fn win(&mut self) {
        self.play(&self.sfx.win.clone());
        if self.best_moves == 0 || self.moves < self.best_moves {
            self.best_moves = self.moves;
        }
        if self.best_time == 0.0 || self.timer < self.best_time {
            self.best_time = self.timer;
        }
        self.state = GameState::Victory;
        self.state_timer = 0.0;
    }

    pub fn draw_playing(&self) {
        let (ox, oy) = self.board_origin();
        self.draw_board_bg(ox, oy);
        self.draw_all_tiles(ox, oy);
        self.draw_hud(ox, oy);

        let hint = "ARROWS / WASD / CLICK    ESC  MENU";
        let cx = screen_width() / 2.0;
        let hw = self.measure(hint, 14.0).width;
        self.text(hint, cx - hw / 2.0, screen_height() - 20.0, 14.0, TEXT_DIM);

        let mut hovering = false;
        if self.anims.is_empty() {
            let (mx, my) = mouse_position();
            let cs = self.cell_size();
            let col = ((mx - ox) / cs) as i32;
            let row = ((my - oy) / cs) as i32;
            if row >= 0 && row < self.size as i32 && col >= 0 && col < self.size as i32 {
                let (er, ec) = self.board.empty_pos();
                let r = row as usize;
                let c = col as usize;
                if (r == er || c == ec) && !(r == er && c == ec) {
                    hovering = true;
                }
            }
        }
        apply_cursor(hovering);
    }
}
