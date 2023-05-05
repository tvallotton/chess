use std::mem::size_of;

use crate::{
    location::Location,
    metadata::Metadata,
    piece::{Color::*, Kind::*, Piece},
};

#[derive(Clone, Copy)]
pub struct Board {
    white: Player,
    black: Player,
    pub meta: Metadata,
}

#[derive(Clone, Copy)]
pub struct Player {
    king: Option<Location>,
    queen: Option<Location>,
    bishop: [Option<Location>; 2],
    knight: [Option<Location>; 2],
    rook: [Option<Location>; 2],
    pawn: [Option<Location>; 8],
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

impl Player {
    fn white() -> Self {
        Player {
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
