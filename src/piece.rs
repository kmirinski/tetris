use ratatui::style::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    I, O, T, S, Z, J, L,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub rotation: usize,
    pub x: i32,
    pub y: i32,
}

impl PieceType {
    pub fn cells(&self, rotation: usize) -> [(i32, i32); 4] {
        let r = rotation % 4;
        match self {
            PieceType::I => [
                [(0,0), (0,1), (0,2), (0,3)],
                [(0,0), (1,0), (2,0), (3,0)],
                [(0,0), (0,1), (0,2), (0,3)],
                [(0,0), (1,0), (2,0), (3,0)],
            ][r],
            PieceType::O => [
                [(0,0), (0,1), (1,0), (1,1)],
                [(0,0), (0,1), (1,0), (1,1)],
                [(0,0), (0,1), (1,0), (1,1)],
                [(0,0), (0,1), (1,0), (1,1)],
            ][r],
            PieceType::T => [
                [(0,0), (0,1), (0,2), (1,1)],
                [(0,0), (1,0), (2,0), (1,1)],
                [(1,0), (1,1), (1,2), (0,1)],
                [(0,0), (1,0), (2,0), (1,-1)],
            ][r],
            PieceType::S => [
                [(0,1), (0,2), (1,0), (1,1)],
                [(0,0), (1,0), (1,1), (2,1)],
                [(0,1), (0,2), (1,0), (1,1)],
                [(0,0), (1,0), (1,1), (2,1)],
            ][r],
            PieceType::Z => [
                [(0,0), (0,1), (1,1), (1,2)],
                [(0,1), (1,0), (1,1), (2,0)],
                [(0,0), (0,1), (1,1), (1,2)],
                [(0,1), (1,0), (1,1), (2,0)],
            ][r],
            PieceType::J => [
                [(0,0), (1,0), (1,1), (1,2)],
                [(0,0), (0,1), (1,0), (2,0)],
                [(0,0), (0,1), (0,2), (1,2)],
                [(0,0), (1,0), (2,0), (2,-1)],
            ][r],
            PieceType::L => [
                [(0,2), (1,0), (1,1), (1,2)],
                [(0,0), (1,0), (2,0), (2,1)],
                [(0,0), (0,1), (0,2), (1,0)],
                [(0,0), (0,1), (1,1), (2,1)],
            ][r],
        }
    }

    pub fn color(&self) -> Color {
        match self {
            PieceType::I => Color:: Cyan,
            PieceType::O => Color:: Yellow,
            PieceType::T => Color:: Magenta,
            PieceType::S => Color::Green,
            PieceType::Z => Color::Red,
            PieceType::J => Color::Blue,
            PieceType::L => Color::Rgb(255, 165, 0),
        }
    }
}

impl Piece {
    pub fn new(piece_type: PieceType) -> Self {
        Self {
            piece_type,
            rotation: 0,
            x: 3,
            y: 0
        }
    }
    pub fn cells(&self) -> [(i32, i32); 4] {
        self.piece_type.cells(self.rotation).map(|(r, c)| (self.y + r, self.x + c))
    }

    pub fn rotated(&self) -> Self {
        Self {
            rotation: (self.rotation + 1) % 4,
            ..*self
        }
    }

    pub fn moved(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            ..*self
        }
    }
}
