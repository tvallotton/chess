use std::mem::size_of;

use crate::{
    location::Location,
    metadata::Metadata,
    moves::{Color, Move, Positions},
    piece::{Color::*, Piece},
};

#[derive(Clone, Copy)]
pub struct Board {
    pub white: Player,
    pub black: Player,
    pub meta: Metadata,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Player {
    pub royalty: [Option<Location>; 2],
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
    pub fn me_mut(&mut self) -> &mut Player {
        match self.meta.turn() {
            White => &mut self.white,
            Black => &mut self.black,
        }
    }
    #[inline]
    pub fn opponent_mut(&mut self) -> &mut Player {
        match self.meta.turn() {
            White => &mut self.black,
            Black => &mut self.white,
        }
    }
    #[inline]
    pub fn opponent(&self) -> &Player {
        match self.meta.turn() {
            White => &self.black,
            Black => &self.white,
        }
    }

    pub fn apply(&self, mov: Move, pos: &Positions) -> Board {
        let mut board = *self;

        let loc = unsafe {
            board
                .me_mut()
                .get(mov.from)
                .as_mut()
                .unwrap()
        };
        *loc = mov.to;
        board
    }
}

impl Player {
    unsafe fn get(&mut self, piece: Piece) -> &mut Option<Location> {
        let locations = self as *mut Player as *mut Option<Location>;
        &mut *locations.offset(piece.0 as isize)
    }

    fn remove(&mut self, loc: Location) {
        let rm_list = |pieces: &mut [_]| {
            pieces
                .iter_mut()
                .for_each(|piece| {
                    if *piece == Some(loc) {
                        *piece = None
                    }
                });
        };
        rm_list(&mut self.royalty);
        rm_list(&mut self.bishop);
        rm_list(&mut self.knight);
        rm_list(&mut self.rook);
        rm_list(&mut self.pawn);
    }

    fn white() -> Self {
        Player {
            color: White,
            royalty: [(7, 4), (7, 3)]
                .map(Into::into)
                .map(Some),
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
            royalty: [(0, 4), (0, 3)]
                .map(Into::into)
                .map(Some),

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
