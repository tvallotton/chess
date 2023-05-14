pub use super::piece::{Color, *};
use crate::moves::utils::debug;
use crate::{board, MoveCache};
use crate::{board::Board, location::Location};
use std::iter::from_fn;
use std::ops::ShlAssign;

pub use self::bitfields::Bitfields;
pub use self::positions::Positions;
use self::{king::king_moves, utils::invert};

pub mod bishop;
mod bitfields;
pub mod king;
pub mod knight;
pub mod move_cache;
pub mod pawn;
mod positions;
pub mod queen;
pub mod rook;
mod utils;

pub struct Move {
    pub from: PieceIndex,
    pub to: Location,
}

struct MoveIterator<'a> {
    bit: u64,
    move_cache: MoveCache<'a>,
    piece: PieceIndex,
}

impl<'a> Iterator for MoveIterator<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            bit,
            move_cache,
            piece,
        } = self;
        loop {
            // skip first 8 pieces if its empty
            while (move_cache.all & *bit) == 0 {
                if *bit == 0 {
                    return None;
                }
                *bit <<= 1;
            }
            if *bit == 0 {
                return None;
            }
            log::debug!("asd");
            // skip first 8 pieces if its empty
            let first_8 = (move_cache.halves[0] & *bit) == 0;
            piece.at_least(8 * first_8 as usize);

            while !piece.finished() {
                let moves = move_cache.get(*piece);
                let from = *piece;
                piece.next();

                // // skip second 8 pieces if its empty
                // let second_8 = (move_cache.halves[1] & *bit) == 0;
                // piece.at_least(16 * second_8 as usize);
                if (piece.kind() == Kind::Knight && from.kind() == Kind::Knight && from.0 == 4) {
                    log::debug!(
                        "moves: {moves} bit: {bit:?} loc: {:?} piece: {from:?} intersect: {}",
                        Location::from_pos(*bit),
                        moves & *bit
                    );
                    debug(moves);
                    debug(*bit);
                }

                if (moves & *bit) != 0 {
                    log::debug!("YESS");
                    return Some(Move {
                        from,
                        to: Location::from_pos(*bit),
                    });
                }
            }

            *piece = KING;
            *bit <<= 1;
        }
    }
}

fn _moves<'a>(board: &'a Board, pos: &Positions) -> impl Iterator<Item = Move> + 'a {
    let move_cache = MoveCache::new(board, &pos);
    let bit = 1;
    let piece = KING;
    MoveIterator {
        bit,
        move_cache,
        piece,
    }
}
pub fn moves(board: &Board) -> impl Iterator<Item = Move> + '_ {
    let pos = Positions::from_board(board);

    debug(pos.mine);
    debug(pos.opponent);
    _moves(board, &pos)
}

pub fn children(board: &Board) -> impl Iterator<Item = Board> + '_ {
    let pos = Positions::from_board(board);
    _moves(board, &pos).map(move |mov| board._apply(mov, &pos))
}
