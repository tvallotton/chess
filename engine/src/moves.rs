pub use super::piece::{Color, *};
use crate::{board::Board, location::Location};
use std::iter::from_fn;

use self::positions::Positions;
use self::{king::king_moves, utils::invert};

pub mod bishop;
mod bitfields;
pub mod king;
pub mod knight;
pub mod pawn;
mod positions;
pub mod queen;
pub mod rook;
mod utils;

pub struct Move {
    from: Location,
    to: Location,
}

pub struct Bitfields {
    king: u64,
    queen: u64,
    knight: [u64; 2],
    pawn: [u64; 2],
    rook: [u64; 2],
    bishop: [u64; 2],
}

pub fn moves(_board: Board) -> impl Iterator<Item = Move> {
    let mut bit = 1;
    let mut kind = PAWN;
    let mut piece = 0;

    from_fn(move || {
        bit <<= 1;
        None
    })
}
