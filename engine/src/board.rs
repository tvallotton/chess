use std::mem::size_of;

use crate::{
    location::Location,
    metadata::Metadata,
    moves::Color,
    piece::{Color::*, Piece},
};

#[derive(Clone, Copy)]
pub struct Board {
    pub white: Player,
    pub black: Player,
    pub meta: Metadata,
}

#[derive(Clone, Copy)]
pub struct Player {
    pub king: Option<Location>,
    pub queen: Option<Location>,
    pub bishop: [Option<Location>; 2],
    pub knight: [Option<Location>; 2],
    pub rook: [Option<Location>; 2],
    pub pawn: [Option<Location>; 8],
    pub color: Color,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            white: Player::white(),
            black: Player::black(),
            meta: Metadata::default(),
        }
    }
}

impl Board {
    #[inline]
    pub fn me(&self) -> &Player {
        match self.meta.turn() {
            White => &self.white,
            Black => &self.black,
        }
    }
    #[inline]
    pub fn opponent(&self) -> &Player {
        match self.meta.turn() {
            White => &self.black,
            Black => &self.white,
        }
    }
}

impl Player {
    fn white() -> Self {
        Player {
            color: White,
            king: Some((7, 4).into()),
            queen: Some((7, 3).into()),
            bishop: [(7, 3), (7, 5)]
                .map(Into::into)
                .map(Some),
            knight: [(7, 2), (7, 6)]
                .map(Into::into)
                .map(Some),
            rook: [(7, 0), (7, 7)]
                .map(Into::into)
                .map(Some),
            pawn: [
                (6, 0),
                (6, 1),
                (6, 2),
                (6, 3),
                (6, 4),
                (6, 5),
                (6, 6),
                (6, 7),
            ]
            .map(Into::into)
            .map(Some),
        }
    }
    fn black() -> Self {
        Player {
            color: Black,
            king: Some((0, 4).into()),
            queen: Some((0, 3).into()),
            bishop: [(0, 3), (0, 5)]
                .map(Into::into)
                .map(Some),
            knight: [(0, 2), (0, 6)]
                .map(Into::into)
                .map(Some),
            rook: [(0, 0), (0, 7)]
                .map(Into::into)
                .map(Some),
            pawn: [
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (1, 6),
                (1, 7),
            ]
            .map(Into::into)
            .map(Some),
        }
    }
}

#[test]
fn foo() {
    println!("board1 {}", size_of::<Board>());
}
