use std::ops::{BitOr};


pub use Color::*;
pub use Kind::*;
#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
}

#[derive(Clone, Copy, Debug)]
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
    pub fn pawn_blockking_rank(self) -> isize {
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
pub const WH_ROOK: Piece = Piece {
    kind: Rook,
    color: White,
};
pub const WH_BISHOP: Piece = Piece {
    kind: Bishop,
    color: White,
};
pub const WH_KING: Piece = Piece {
    kind: King,
    color: White,
};
pub const WH_QUEEN: Piece = Piece {
    kind: Queen,
    color: White,
};
pub const WH_KNIGHT: Piece = Piece {
    kind: Knight,
    color: White,
};
pub const WH_PAWN: Piece = Piece {
    kind: Pawn,
    color: White,
};

pub const BL_ROOK: Piece = Piece {
    kind: Rook,
    color: Black,
};
pub const BL_BISHOP: Piece = Piece {
    kind: Bishop,
    color: Black,
};
pub const BL_KING: Piece = Piece {
    kind: King,
    color: Black,
};
pub const BL_QUEEN: Piece = Piece {
    kind: Queen,
    color: Black,
};
pub const BL_KNIGHT: Piece = Piece {
    kind: Knight,
    color: Black,
};
pub const BL_PAWN: Piece = Piece {
    kind: Pawn,
    color: Black,
};
