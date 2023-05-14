#![allow(clippy::all)]
use std::mem::transmute;

use crate::{
    bishop_moves,
    board::{Board, Player},
    king_moves, knight_moves,
    location::Location,
    moves::bishop,
    pawn_moves, queen_moves, rook_moves,
};

use super::{utils::or, PieceIndex, Positions};

#[repr(C)]
pub struct MoveCache<'a> {
    pub royalty: [u64; 2],
    pub bishop: [u64; 2],
    pub knight: [u64; 2],
    pub rook: [u64; 2],
    pub pawn: [u64; 8],
    pub all: u64,
    pub halves: [u64; 2],

    pub player: &'a Player,
}

impl<'a> MoveCache<'a> {
    pub fn get(&self, piece: PieceIndex) -> u64 {
        let bitfields = self as *const Self as *const u64;
        unsafe { *bitfields.offset(piece.0 as isize) }
    }

    pub fn get_loc(&self, piece: PieceIndex) -> Option<Location> {
        let locations = self.player as *const Player as *const Option<Location>;
        unsafe { *locations.offset(piece.0 as isize) }
    }

    pub fn new(board: &'a Board, pos: &Positions) -> Self {
        let me = board.me();

        let king = me.royalty[0]
            .map(|king| king_moves(pos, king))
            .unwrap_or_default();

        let queen = me.royalty[1]
            .map(|queen| queen_moves(pos, queen))
            .unwrap_or_default();

        let rook = me.rook.map(|rooks| {
            rooks
                .map(|rook| rook_moves(pos, rook))
                .unwrap_or_default()
        });
        let bishop = me.bishop.map(|bishop| {
            bishop
                .map(|bishop| bishop_moves(pos, bishop))
                .unwrap_or_default()
        });
        let knight = me.knight.map(|knight| {
            knight
                .map(|knight| knight_moves(pos, knight))
                .unwrap_or_default()
        });

        let pawn = me.pawn.map(|pawn| {
            pawn.map(|pawn| {
                if pawn.is_queen() {
                    queen_moves(pos, pawn)
                } else {
                    pawn_moves(pos, pawn, me.color)
                }
            })
            .unwrap_or_default()
        });

        let halves = [king | queen | or(bishop) | or(rook) | or(knight), or(pawn)];

        let all = halves[0] | halves[1];

        MoveCache {
            royalty: [king, queen],
            bishop,
            rook,
            knight,
            pawn,
            all,
            halves,
            player: me,
        }
    }
}
