use macroquad::prelude::*;

use crate::consts::*;
use crate::game::Game;

fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);
    draw_circle(x + r, y + r, r, color);
    draw_circle(x + w - r, y + r, r, color);
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color);
}

impl Game {
    pub fn draw_board_bg(&self, ox: f32, oy: f32) {
        let cs = self.cell_size();
        let total = self.size as f32 * cs;
        let pad = 8.0;

        draw_rounded_rect(
            ox - pad + 3.0,
            oy - pad + 3.0,
            total + pad * 2.0,
            total + pad * 2.0,
            12.0,
            Color::new(0.0, 0.0, 0.0, 0.06),
        );
        draw_rounded_rect(
            ox - pad,
            oy - pad,
            total + pad * 2.0,
            total + pad * 2.0,
            12.0,
            BOARD_BG,
        );

        for r in 0..self.size {
            for c in 0..self.size {
                let x = ox + c as f32 * cs + TILE_GAP / 2.0;
                let y = oy + r as f32 * cs + TILE_GAP / 2.0;
                let s = cs - TILE_GAP;
                draw_rounded_rect(x, y, s, s, TILE_ROUND, Color::new(0.0, 0.0, 0.0, 0.04));
            }
        }
    }

    pub fn draw_tile(&self, ox: f32, oy: f32, row: usize, col: usize, value: u8) {
        if value == 0 {
            return;
        }
        let cs = self.cell_size();
        let gap = TILE_GAP;
        let round = TILE_ROUND;

        let (mut px, mut py) = (col as f32, row as f32);

        for a in &self.anims {
            if a.to_row == row && a.to_col == col {
                let t = a.t.clamp(0.0, 1.0);
                let ease = t * t * (3.0 - 2.0 * t);
                px = a.from_col as f32 + (col as f32 - a.from_col as f32) * ease;
                py = a.from_row as f32 + (row as f32 - a.from_row as f32) * ease;
                break;
            }
        }

        let x = ox + px * cs + gap / 2.0;
        let y = oy + py * cs + gap / 2.0;
        let s = cs - gap;

        draw_rounded_rect(x + 2.0, y + 2.0, s, s, round, TILE_SHADOW);

        let bg = if self.board.is_correct(row, col) {
            Color::new(
                TILE_COLOR.r * 0.9 + TILE_CORRECT.r * 0.1,
                TILE_COLOR.g * 0.9 + TILE_CORRECT.g * 0.1,
                TILE_COLOR.b * 0.9 + TILE_CORRECT.b * 0.1,
                1.0,
            )
        } else {
            TILE_COLOR
        };
        draw_rounded_rect(x, y, s, s, round, bg);

        draw_rounded_rect(x, y, s, 2.0_f32.min(s * 0.06), round, Color::new(1.0, 1.0, 1.0, 0.4));

        let num = format!("{}", value);
        let font_size = (cs * 0.45).max(16.0);
        let dims = self.measure(&num, font_size);
        let tx = x + (s - dims.width) / 2.0;
        let ty = y + s / 2.0 + dims.height / 2.0 - 2.0;

        let text_color = if self.board.is_correct(row, col) {
            Color::new(0.25, 0.55, 0.25, 1.0)
        } else {
            TILE_TEXT
        };
        self.text(&num, tx, ty, font_size, text_color);
    }

    pub fn draw_all_tiles(&self, ox: f32, oy: f32) {
        for r in 0..self.size {
            for c in 0..self.size {
                let v = self.board.tile_at(r, c);
                self.draw_tile(ox, oy, r, c, v);
            }
        }
    }

    pub fn draw_panel(&self, cx: f32, cy: f32, w: f32, h: f32) {
        let x = cx - w / 2.0;
        let y = cy - h / 2.0;
        draw_rounded_rect(x + 3.0, y + 3.0, w, h, 14.0, Color::new(0.0, 0.0, 0.0, 0.08));
        draw_rounded_rect(x, y, w, h, 14.0, BG);
        draw_rounded_rect(x + 1.0, y + 1.0, w - 2.0, h - 2.0, 13.0, Color::new(0.88, 0.85, 0.80, 0.5));
    }

    pub fn draw_hud(&self, ox: f32, oy: f32) {
        let cs = self.cell_size();
        let total = self.size as f32 * cs;
        let top_y = oy - 40.0;

        let moves_str = format!("Moves: {}", self.moves);
        self.text(&moves_str, ox, top_y, 22.0, TEXT_DARK);

        let time_str = Game::format_time(self.timer);
        let tw = self.measure(&time_str, 22.0).width;
        self.text(&time_str, ox + total - tw, top_y, 22.0, TEXT_DARK);
    }
}
