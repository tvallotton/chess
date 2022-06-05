use std::{fmt::Display, ops::BitOr, str::FromStr};

pub use Color::*;
pub use Kind::*;
use serde::Deserialize;
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all= "lowercase")]
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

impl FromStr for Color {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("white") {
            Ok(White)
        } else if s.eq_ignore_ascii_case("black") {
            Ok(Black)
        } else {
            Err("not a valid player")
        }
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
            Black => 7,
            White => 0,
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
impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bishop => write!(f, "♝")?,
            Rook => write!(f, "♜")?,
            King => write!(f, "♚")?,
            Queen => write!(f, "♛")?,
            Pawn => write!(f, "♟")?,
            Knight => write!(f, "♞")?,
        }
        Ok(())
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            White => write!(f, "white"),
            Black => write!(f, "black"),
        }
    }
}
