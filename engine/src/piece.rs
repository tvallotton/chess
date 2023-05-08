use std::{
    fmt::{Debug, Display},
    mem::transmute,
    num::NonZeroU8,
    ops::BitOr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White = 0b10,
    Black = 0b11,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy)]
pub struct Piece(pub(super) u8);

pub const KING: Piece = Piece(0);
pub const QUEEN: Piece = Piece(1);
pub const BISHOP: Piece = Piece(2);
pub const KNIGHT: Piece = Piece(4);
pub const ROOK: Piece = Piece(6);
pub const PAWN: Piece = Piece(8);

impl Piece {
    #[inline]
    pub fn next(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    pub fn finished(&mut self) -> bool {
        self.0 >= 16
    }

    #[inline]
    pub fn at_least(&mut self, val: usize) {
        self.0 = self.0.max(val as u8);
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Black => write!(f, "black"),
            Color::White => write!(f, "white"),
        }
    }
}
