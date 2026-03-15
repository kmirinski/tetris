use  ratatui::style::Color;
use crate::piece::Piece;

pub struct Board{
    cells: [[Option<Color>; 10]; 20],
}

impl Board {
    pub const WIDTH: i32 = 10;
    pub const HEIGHT: i32 = 20;

    pub fn new() -> Self {
        Self {
            cells: [[None; 10]; 20],
        }
    }

    pub fn is_valid(&self, piece: &Piece) -> bool {
        for (row, col) in piece.cells() {
            if col < 0 || col >= Self::WIDTH {
                return false;
            }
            if row >= Self::HEIGHT {
                return false;
            }
            if row >= 0 && self.cells[row as usize][col as usize].is_some() {
                return false;
            }
        }
        return true;
    }

    pub fn lock(&mut self, piece: &Piece) {
        let color = piece.piece_type.color();
        for (row, col) in piece.cells() {
            if row >= 0 {
                self.cells[row as usize][col as usize] = Some(color);
            }
        }
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut remaining: Vec<[Option<Color>; 10]> = self.cells
            .iter()
            .filter(|row| !row.iter().all(|c| c.is_some()))
            .copied()
            .collect();

        let cleared = (Self::HEIGHT as u32) - remaining.len() as u32;

        let mut new_cells = [[None; 10]; 20];
        for (i, row) in remaining.drain(..).enumerate() {
            new_cells[cleared as usize + i] = row;
        }
        self.cells = new_cells;

        cleared
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Color> {
        self.cells[row][col]
    }
}