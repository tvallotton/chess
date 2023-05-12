use std::ops::Index;

use crate::{
    location::Location,
    metadata::Metadata,
    moves::{children, moves, Color, Kind, Move, Piece, Positions, BISHOP},
    piece::{Color::*, PieceIndex},
    search::minimax,
    Params,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Board {
    pub white: Player,
    pub black: Player,
    pub meta: Metadata,
}

#[derive(Clone, Copy, PartialEq, Debug)]
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

    pub fn apply(&self, (to, from): (Location, Location)) -> Option<Board> {
        let pos = &Positions::from_board(self);
        let from = self.search_for(from)?.index;
        let mov = Move { to, from };
        Some(self._apply(mov, pos))
    }

    pub fn _apply(&self, mov: Move, pos: &Positions) -> Board {
        let mut board = *self;

        let loc = unsafe {
            board
                .me_mut()
                .get_mut(mov.from)
                .as_mut()
                .unwrap_unchecked()
        };
        *loc = mov.to;

        let is_capture = (pos.opponent & mov.to.pos()) != 0;

        if is_capture {
            board
                .opponent_mut()
                .remove(mov.to);
        }

        board
    }

    pub fn children(&self) -> impl Iterator<Item = Board> + '_ {
        children(self)
    }
}

impl Player {
    unsafe fn get_mut(&mut self, piece: PieceIndex) -> &mut Option<Location> {
        &mut self.locations_mut()[piece.0 as usize]
    }
    unsafe fn get(&self, piece: PieceIndex) -> &Option<Location> {
        &self.locations()[piece.0 as usize]
    }

    pub fn locations(&self) -> &[Option<Location>; 16] {
        unsafe { &*(self as *const Player as *const [Option<Location>; 16]) }
    }

    pub fn locations_mut(&mut self) -> &mut [Option<Location>; 16] {
        unsafe { &mut *(self as *mut Player as *mut [Option<Location>; 16]) }
    }

    fn remove(&mut self, loc: Location) {
        fn rm_list<const D: usize>(pieces: &mut [Option<Location>; D], loc: Location) {
            for piece in pieces {
                if *piece == Some(loc) {
                    *piece = None
                }
            }
        }

        rm_list(&mut self.royalty, loc);
        rm_list(&mut self.bishop, loc);
        rm_list(&mut self.knight, loc);
        rm_list(&mut self.rook, loc);
        rm_list(&mut self.pawn, loc);
    }

    pub fn white() -> Self {
        Player {
            color: White,
            royalty: [(7, 4), (7, 3)]
                .map(Into::into)
                .map(Some),
            bishop: [(7, 2), (7, 5)]
                .map(Into::into)
                .map(Some),
            knight: [(7, 1), (7, 6)]
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

            bishop: [(0, 2), (0, 5)]
                .map(Into::into)
                .map(Some),
            knight: [(0, 1), (0, 6)]
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

impl Board {
    pub fn play_with(&self, params: &Params) -> Board {
        let mut children: Vec<_> = self.children().collect();
        children.sort_by_cached_key(|board| {
            let score = minimax(board, params.depth, i32::MIN, i32::MAX, params);
            if board.me().color == White {
                -score
            } else {
                score
            }
        });
        children[0]
    }

    pub fn search_for(self, index: Location) -> Option<Piece> {
        let find = |p: &Player| {
            let (index, _) = p
                .locations()
                .into_iter()
                .enumerate()
                .find(|(_, x)| **x == Some(index))?;
            let index = PieceIndex(index as u8);
            Some(Piece {
                kind: index.kind(),
                color: p.color,
                index,
            })
        };

        let me = self.me();
        find(self.me()).or_else(|| find(self.opponent()))
    }

    pub fn moves_for_piece(&self, loc: Location) -> impl Iterator<Item = Move> + '_ {
        moves(self).filter(move |mov| unsafe { *self.me().get(mov.from) } == Some(loc))
    }
}

#[test]
fn foo() {
    println!("board1 {}", std::mem::size_of::<Board>());
}
