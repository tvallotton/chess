use super::Positions;
use crate::{location::Location, piece::Color};

pub struct PawnMoves([u64; 2]);

impl PawnMoves {}

pub fn pawn_moves(pos: &Positions, loc: u64) -> PawnMoves {
    loc << 8;

    todo!()
}
#[inline]
pub fn left_captures(pos: &Positions, pawns: u64, color: Color) -> u64 {
    let white = (color == Color::White) as u8;
    let black = (color == Color::Black) as u8;
    pawns >> (9 * white) << (9 * black)
}

#[inline]
pub fn right_captures(pos: &Positions, pawns: u64, color: Color) -> u64 {
    let white = (color == Color::White) as u8;
    let black = (color == Color::Black) as u8;
    pawns >> (7 * white) << (7 * black)
}
