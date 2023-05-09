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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
    pub index: PieceIndex,
}

impl Piece {
    pub fn piece_iter(&self) -> PieceIndex {
        match self.kind {
            Kind::King => KING,
            Kind::Queen => QUEEN,
            Kind::Bishop => BISHOP,
            Kind::Knight => KNIGHT,
            Kind::Rook => ROOK,
            Kind::Pawn => PAWN,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PieceIndex(pub(super) u8);

pub const KING: PieceIndex = PieceIndex(0);
pub const QUEEN: PieceIndex = PieceIndex(1);
pub const BISHOP: PieceIndex = PieceIndex(2);
pub const KNIGHT: PieceIndex = PieceIndex(4);
pub const ROOK: PieceIndex = PieceIndex(6);
pub const PAWN: PieceIndex = PieceIndex(8);

impl PieceIndex {
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

    pub fn kind(&self) -> Kind {
        match self.0 {
            0 => Kind::King,
            1 => Kind::Queen,
            2 | 3 => Kind::Bishop,
            4 | 5 => Kind::Knight,
            6 | 7 => Kind::Rook,
            _ => Kind::Pawn,
        }
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

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::King => write!(f, "king"),
            Kind::Queen => write!(f, "queen"),
            Kind::Bishop => write!(f, "bishop"),
            Kind::Knight => write!(f, "knight"),
            Kind::Rook => write!(f, "rook"),
            Kind::Pawn => write!(f, "pawn"),
        }
    }
}
