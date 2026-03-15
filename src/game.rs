use rand::{Rng, RngExt};

use crate::board::Board;
use crate::piece::{Piece, PieceType};

const PIECES: [PieceType; 7] = [
    PieceType::I,
    PieceType::O,
    PieceType::T,
    PieceType::S,
    PieceType::Z,
    PieceType::J,
    PieceType::L,
];

fn pick_piece(last: PieceType) -> PieceType {
    let mut rng = rand::rng();
    let idx = rng.random_range(0usize..8);

    if idx == 7 || PIECES[idx] == last {
        PIECES[rng.random_range(0usize..7)]
    } else {
        PIECES[idx]
    }
}

pub struct Game {
    pub board: Board,
    pub current: Piece,
    pub queue: [PieceType; 3],
    pub score: u32,
    pub lines: u32,
    pub level: u32,
    pub game_over: bool,
    start_level: u32,
}

impl Game {
    pub fn new (start_level: u32) -> Self {
        let mut rng = rand::rng();
        let first = PIECES[rng.random_range(0..7)];
        let q0 = pick_piece(first);
        let q1 = pick_piece(q0);
        let q2 = pick_piece(q1);

        Self {
            board: Board::new(),
            current: Piece::new(first),
            queue: [q0, q1, q2],
            score: 0,
            lines: 0,
            level: start_level,
            game_over: false,
            start_level,
        }
    }

    pub fn tick(&mut self) {
        if self.game_over {
            return
        }
        let moved = self.current.moved(0, 1);
        if self.board.is_valid(&moved) {
            self.current = moved;
        } else {
            self.lock_and_spawn();
        }
    }

    pub fn move_left(&mut self) {
        if self.game_over {
            return
        }
        let moved = self.current.moved(-1, 0);
        if self.board.is_valid(&moved) {
            self.current = moved;
        }
    }

    pub fn move_right(&mut self) {
        if self.game_over {
            return
        }
        let moved = self.current.moved(1, 0);
        if self.board.is_valid(&moved) {
            self.current = moved;
        }
    }

    pub fn rotate(&mut self) {
        if self.game_over {
            return
        }
        let rotated = self.current.rotated();
        if self.board.is_valid(&rotated) {
            self.current = rotated;
        }
    }

    pub fn soft_drop(&mut self) {
        if self.game_over {
            return
        }
        let moved = self.current.moved(0, 1);
        if self.board.is_valid(&moved) {
            self.current = moved;
            self.score += 1;
        } else {
            self.lock_and_spawn();
        }
    }

    pub fn hard_drop(&mut self) {
        if self.game_over {
            return
        }
        loop {
            let moved = self.current.moved(0, 1);
            if self.board.is_valid(&moved) {
                self.current = moved;
            } else {
                break;
            }
        }
        self.lock_and_spawn();
    }

    pub fn gravity_ms(&self) -> u64 {
        let frames: u64 = match self.level {
            0 => 48,
            1 => 43,
            2 => 38,
            3 => 33,
            4 => 28,
            5 => 23,
            6 => 18,
            7 => 13,
            8 => 8,
            9 => 6,
            10..=12 => 5,
            13..=15 => 4,
            16..=18 => 3,
            19..=28 => 2,
            _ => 1,
        };
        frames * 1000 / 60
    }

    fn lock_and_spawn(&mut self) {
        self.board.lock(&self.current);
        let cleared = self.board.clear_lines();
        self.score += Self::line_score(cleared, self.level);
        self.lines += cleared;
        self.level = self.start_level.max(self.lines / 10);
        self.spawn_next();
    }

    fn spawn_next(&mut self) {
        self.current = Piece::new(self.queue[0]);
        let new_type = pick_piece(self.queue[2]);
        self.queue = [self.queue[1], self.queue[2], new_type];
        if !self.board.is_valid(&self.current) {
            self.game_over = true;
        }
    }

    fn line_score(cleared: u32, level:u32) -> u32 {
        let base: u32 = match cleared {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => 0
        };
        base * (level + 1)
    }
}