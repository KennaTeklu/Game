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

const COLORS: [u32; 8] = [
    0x000000, // empty
    0x00ffff, // I cyan
    0xffff00, // O yellow
    0xaa66ff, // T purple
    0x00ff00, // S green
    0xff0000, // Z red
    0xffaa00, // L orange
    0x0000ff, // J blue
];

#[wasm_bindgen]
pub struct Tetris {
    board: [[u8; WIDTH]; HEIGHT],
    piece_x: i32,
    piece_y: i32,
    piece_type: usize,
    piece_rotation: usize,
    next_piece: usize,
    score: u32,
    lines: u32,
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
            next_piece: 0,
            score: 0,
            lines: 0,
            game_over: false,
        };
        tetris.next_piece = (js_sys::Math::random() * 7.0) as usize;
        tetris.spawn_new_piece();
        tetris
    }

    fn spawn_new_piece(&mut self) {
        self.piece_type = self.next_piece;
        self.next_piece = (js_sys::Math::random() * 7.0) as usize;
        self.piece_rotation = 0;
        self.piece_x = 3;
        self.piece_y = 0;
        if self.collision() {
            self.game_over = true;
        }
    }

    fn get_piece_shape(&self) -> &'static [[i32; 4]; 4] {
        &SHAPES[self.piece_type]
    }

    fn get_rotated_shape(&self) -> [[i32; 4]; 4] {
        let shape = self.get_piece_shape();
        let mut rotated = [[0; 4]; 4];
        match self.piece_rotation % 4 {
            0 => rotated = *shape,
            1 => {
                for i in 0..4 {
                    for j in 0..4 {
                        rotated[j][3 - i] = shape[i][j];
                    }
                }
            }
            2 => {
                for i in 0..4 {
                    for j in 0..4 {
                        rotated[3 - i][3 - j] = shape[i][j];
                    }
                }
            }
            3 => {
                for i in 0..4 {
                    for j in 0..4 {
                        rotated[3 - j][i] = shape[i][j];
                    }
                }
            }
            _ => (),
        }
        rotated
    }

    fn collision_with_shape(&self, shape: &[[i32; 4]; 4], x: i32, y: i32) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if shape[i][j] != 0 {
                    let nx = x + j as i32;
                    let ny = y + i as i32;
                    if nx < 0 || nx >= WIDTH as i32 || ny >= HEIGHT as i32 || ny < 0 {
                        return true;
                    }
                    if ny >= 0 && self.board[ny as usize][nx as usize] != 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn collision(&self) -> bool {
        let shape = self.get_rotated_shape();
        self.collision_with_shape(&shape, self.piece_x, self.piece_y)
    }

    fn merge_piece(&mut self) {
        let shape = self.get_rotated_shape();
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
                for r in (1..=row).rev() {
                    self.board[r] = self.board[r - 1];
                }
                self.board[0] = [0; WIDTH];
                lines_cleared += 1;
                // stay on same row
            } else {
                row -= 1;
            }
        }
        if lines_cleared > 0 {
            self.lines += lines_cleared;
            let points = match lines_cleared {
                1 => 100,
                2 => 300,
                3 => 500,
                4 => 800,
                _ => 0,
            };
            self.score += points;
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
            let original_rot = self.piece_rotation;
            self.piece_rotation = (self.piece_rotation + 1) % 4;
            if self.collision() {
                self.piece_rotation = original_rot;
            }
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
            let shape = self.get_rotated_shape();
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

    pub fn get_next_piece(&self) -> u8 {
        (self.next_piece + 1) as u8
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_lines(&self) -> u32 {
        self.lines
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn reset(&mut self) {
        self.board = [[0; WIDTH]; HEIGHT];
        self.score = 0;
        self.lines = 0;
        self.game_over = false;
        self.next_piece = (js_sys::Math::random() * 7.0) as usize;
        self.spawn_new_piece();
    }
}
