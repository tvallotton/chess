pub use super::piece::{Color, *};
use crate::board;
use crate::{board::Board, location::Location};
use std::iter::from_fn;

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
    from: Location,
    to: Location,
}

pub fn moves(board: Board) -> impl Iterator<Item = Move> {
    let pos = Positions::from_board(board);

    let mut bit = 1;
    let mut kind = PAWN;
    let mut piece = 0;

    from_fn(move || {
        kind += 1;
        bit <<= 1;
        None
    })
}
