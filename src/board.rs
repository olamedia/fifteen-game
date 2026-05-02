use macroquad::rand::gen_range;

#[derive(Clone)]
pub struct Board {
    pub size: usize,
    pub tiles: Vec<u8>,
    pub empty: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let n = size * size;
        let mut tiles: Vec<u8> = (0..n as u8).collect();
        tiles[n - 1] = 0;
        for i in 0..n - 1 {
            tiles[i] = (i + 1) as u8;
        }
        let mut b = Self {
            size,
            tiles,
            empty: n - 1,
        };
        b.shuffle();
        b
    }

    pub fn solved(size: usize) -> Self {
        let n = size * size;
        let mut tiles: Vec<u8> = (0..n as u8).collect();
        for i in 0..n - 1 {
            tiles[i] = (i + 1) as u8;
        }
        tiles[n - 1] = 0;
        Self {
            size,
            tiles,
            empty: n - 1,
        }
    }

    fn inversions(&self) -> usize {
        let n = self.size * self.size;
        let mut inv = 0;
        for i in 0..n {
            if self.tiles[i] == 0 {
                continue;
            }
            for j in i + 1..n {
                if self.tiles[j] == 0 {
                    continue;
                }
                if self.tiles[i] > self.tiles[j] {
                    inv += 1;
                }
            }
        }
        inv
    }

    fn is_solvable(&self) -> bool {
        let inv = self.inversions();
        if self.size % 2 == 1 {
            inv % 2 == 0
        } else {
            let empty_row_from_bottom = self.size - self.empty / self.size;
            (inv + empty_row_from_bottom) % 2 == 1
        }
    }

    fn shuffle(&mut self) {
        let n = self.size * self.size;
        loop {
            for i in (1..n).rev() {
                let j = gen_range(0, i + 1);
                self.tiles.swap(i, j);
            }
            self.empty = self.tiles.iter().position(|&t| t == 0).unwrap();
            if self.is_solvable() && !self.is_solved() {
                break;
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        let n = self.size * self.size;
        for i in 0..n - 1 {
            if self.tiles[i] != (i + 1) as u8 {
                return false;
            }
        }
        self.tiles[n - 1] == 0
    }

    pub fn tile_at(&self, row: usize, col: usize) -> u8 {
        self.tiles[row * self.size + col]
    }

    pub fn empty_pos(&self) -> (usize, usize) {
        (self.empty / self.size, self.empty % self.size)
    }

    pub fn try_move(&mut self, row: usize, col: usize) -> bool {
        let (er, ec) = self.empty_pos();
        let dr = (row as i32 - er as i32).unsigned_abs() as usize;
        let dc = (col as i32 - ec as i32).unsigned_abs() as usize;
        if (dr == 1 && dc == 0) || (dr == 0 && dc == 1) {
            let idx = row * self.size + col;
            self.tiles.swap(self.empty, idx);
            self.empty = idx;
            true
        } else {
            false
        }
    }

    pub fn try_move_dir(&mut self, dr: i32, dc: i32) -> Option<(usize, usize)> {
        let (er, ec) = self.empty_pos();
        let tr = er as i32 - dr;
        let tc = ec as i32 - dc;
        if tr >= 0 && tr < self.size as i32 && tc >= 0 && tc < self.size as i32 {
            let row = tr as usize;
            let col = tc as usize;
            if self.try_move(row, col) {
                return Some((row, col));
            }
        }
        None
    }

    pub fn try_slide(&mut self, row: usize, col: usize) -> Vec<(usize, usize, usize, usize)> {
        let (er, ec) = self.empty_pos();
        if row == er && col == ec {
            return vec![];
        }
        if row != er && col != ec {
            return vec![];
        }
        let mut moves = vec![];
        if row == er {
            let step: i32 = if col < ec { -1 } else { 1 };
            let mut c = ec as i32 + step;
            while (step > 0 && c <= col as i32) || (step < 0 && c >= col as i32) {
                let fc = c as usize;
                let tc = (c - step) as usize;
                self.tiles.swap(row * self.size + fc, row * self.size + tc);
                self.empty = row * self.size + fc;
                moves.push((row, fc, row, tc));
                c += step;
            }
        } else {
            let step: i32 = if row < er { -1 } else { 1 };
            let mut r = er as i32 + step;
            while (step > 0 && r <= row as i32) || (step < 0 && r >= row as i32) {
                let fr = r as usize;
                let tr = (r - step) as usize;
                self.tiles.swap(fr * self.size + col, tr * self.size + col);
                self.empty = fr * self.size + col;
                moves.push((fr, col, tr, col));
                r += step;
            }
        }
        moves
    }

    pub fn is_correct(&self, row: usize, col: usize) -> bool {
        let idx = row * self.size + col;
        let v = self.tiles[idx];
        v != 0 && v == (idx + 1) as u8
    }
}
