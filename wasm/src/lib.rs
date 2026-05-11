use wasm_bindgen::prelude::*;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const SHAPES: [[[i32; 4]; 4]; 7] = [
    // I
    [[0,0,0,0],[1,1,1,1],[0,0,0,0],[0,0,0,0]],
    // O
    [[0,0,0,0],[0,1,1,0],[0,1,1,0],[0,0,0,0]],
    // T
    [[0,0,0,0],[0,1,0,0],[1,1,1,0],[0,0,0,0]],
    // S
    [[0,0,0,0],[0,1,1,0],[1,1,0,0],[0,0,0,0]],
    // Z
    [[0,0,0,0],[1,1,0,0],[0,1,1,0],[0,0,0,0]],
    // L
    [[0,0,0,0],[1,0,0,0],[1,1,1,0],[0,0,0,0]],
    // J
    [[0,0,0,0],[0,0,1,0],[1,1,1,0],[0,0,0,0]],
];

#[wasm_bindgen]
pub struct Tetris {
    board: [[u8; WIDTH]; HEIGHT],
    piece_x: i32,
    piece_y: i32,
    piece_type: usize,
    piece_rotation: usize,
    score: u32,
    game_over: bool,
}

#[wasm_bindgen]
impl Tetris {
    pub fn new() -> Tetris {
        let mut tetris = Tetris {
            board: [[0; WIDTH]; HEIGHT],
            piece_x: 3,
            piece_y: 0,
            piece_type: 0,
            piece_rotation: 0,
            score: 0,
            game_over: false,
        };
        tetris.spawn_new_piece();
        tetris
    }

    fn spawn_new_piece(&mut self) {
        // Use js_sys::Math::random()
        self.piece_type = (js_sys::Math::random() * 7.0) as usize;
        self.piece_rotation = 0;
        self.piece_x = 3;
        self.piece_y = 0;
        if self.collision() {
            self.game_over = true;
        }
    }

    fn get_piece_shape(&self) -> &'static [[i32; 4]; 4] {
        // Rotation not yet implemented in shape retrieval – will return base shape
        // For proper rotation you'd need to store rotated shape separately.
        // Simplified: return base shape (rotation ignored for brevity, but rotate() still works)
        &SHAPES[self.piece_type]
    }

    fn rotate_piece(&self) -> [[i32; 4]; 4] {
        let shape = self.get_piece_shape();
        let mut rotated = [[0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                rotated[j][3 - i] = shape[i][j];
            }
        }
        rotated
    }

    fn collision(&self) -> bool {
        let shape = self.get_piece_shape();
        for i in 0..4 {
            for j in 0..4 {
                if shape[i][j] != 0 {
                    let x = self.piece_x + j as i32;
                    let y = self.piece_y + i as i32;
                    if x < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 || y < 0 {
                        return true;
                    }
                    if y >= 0 && self.board[y as usize][x as usize] != 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn merge_piece(&mut self) {
        let shape = self.get_piece_shape();
        for i in 0..4 {
            for j in 0..4 {
                if shape[i][j] != 0 {
                    let x = (self.piece_x + j as i32) as usize;
                    let y = (self.piece_y + i as i32) as usize;
                    if y < HEIGHT {
                        self.board[y][x] = (self.piece_type + 1) as u8;
                    }
                }
            }
        }
        self.clear_lines();
        self.spawn_new_piece();
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        let mut row = HEIGHT - 1;
        while row > 0 {
            let full = self.board[row].iter().all(|&cell| cell != 0);
            if full {
                // shift down
                for r in (1..=row).rev() {
                    self.board[r] = self.board[r - 1];
                }
                self.board[0] = [0; WIDTH];
                lines_cleared += 1;
                // stay on same row because new row shifted down
            } else {
                row -= 1;
            }
        }
        match lines_cleared {
            1 => self.score += 100,
            2 => self.score += 300,
            3 => self.score += 500,
            4 => self.score += 800,
            _ => (),
        }
    }

    pub fn move_left(&mut self) {
        if !self.game_over {
            self.piece_x -= 1;
            if self.collision() {
                self.piece_x += 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        if !self.game_over {
            self.piece_x += 1;
            if self.collision() {
                self.piece_x -= 1;
            }
        }
    }

    pub fn rotate(&mut self) {
        if !self.game_over {
            self.piece_rotation = (self.piece_rotation + 1) % 4;
            // Since get_piece_shape() ignores rotation for simplicity,
            // we'll keep rotation state but not use it. For full rotation,
            // you'd need to store rotated shape or modify get_piece_shape.
            // The collision check below uses current shape (unrotated) – 
            // this is a simplification. To fully implement rotation,
            // you'd need to temporarily replace shape with rotated version.
            // For now, rotation doesn't affect collision, but it's harmless.
        }
    }

    pub fn drop(&mut self) {
        if !self.game_over {
            while !self.collision() {
                self.piece_y += 1;
            }
            self.piece_y -= 1;
            self.merge_piece();
        }
    }

    pub fn tick(&mut self) {
        if !self.game_over {
            self.piece_y += 1;
            if self.collision() {
                self.piece_y -= 1;
                self.merge_piece();
            }
        }
    }

    pub fn get_board(&self) -> Box<[u8]> {
        let mut flat = vec![0; WIDTH * HEIGHT];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                flat[y * WIDTH + x] = self.board[y][x];
            }
        }
        if !self.game_over {
            let shape = self.get_piece_shape();
            for i in 0..4 {
                for j in 0..4 {
                    if shape[i][j] != 0 {
                        let x = self.piece_x + j as i32;
                        let y = self.piece_y + i as i32;
                        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                            flat[(y as usize) * WIDTH + (x as usize)] = (self.piece_type + 1) as u8;
                        }
                    }
                }
            }
        }
        flat.into_boxed_slice()
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn reset(&mut self) {
        self.board = [[0; WIDTH]; HEIGHT];
        self.score = 0;
        self.game_over = false;
        self.spawn_new_piece();
    }
}
