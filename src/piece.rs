use std::{ops::BitOr, };

pub use Color::*;
pub use Kind::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Default for Color {
    fn default() -> Self {
        White
    }
}

impl BitOr<Kind> for Color {
    type Output = Piece;
    fn bitor(self, kind: Kind) -> Self::Output {
        Piece { kind, color: self }
    }
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Black => White,
            White => Black,
        }
    }
    pub fn promotion_rank(self) -> isize {
        match self {
            Black => 0,
            White => 7,
        }
    }
    pub fn pawn_blocking_rank(self) -> isize {
        match self {
            Black => 2,
            White => 5,
        }
    }
    pub fn pawn_start(self) -> isize {
        match self {
            Black => 1,
            White => 6,
        }
    }
    pub fn pawn_dir(self) -> isize {
        match self {
            Black => 1,
            White => -1,
        }
    }
}
