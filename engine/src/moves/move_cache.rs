#![allow(clippy::all)]
use std::mem::transmute;

use crate::{
    bishop_moves,
    board::{Board, Player},
    king_moves,
    location::Location,
    moves::bishop,
    pawn_moves, queen_moves, rook_moves,
};

use super::{utils::or, Piece, Positions};

#[repr(C)]
pub struct MoveCache<'a> {
    pub king: u64,
    pub queen: u64,
    pub bishop: [u64; 2],
    pub knight: [u64; 2],
    pub rook: [u64; 2],
    pub pawn: [u64; 8],
    pub all: u64,
    pub halves: [u64; 2],
    pub quarters: [[u64; 2]; 2],
    pub player: &'a Player,
}

impl<'a> MoveCache<'a> {
    pub fn get(&self, piece: Piece) -> u64 {
        let bitfields = self as *const Self as *const u64;
        unsafe { *bitfields.offset(piece.0 as isize) }
    }

    pub fn get_loc(&self, piece: Piece) -> Option<Location> {
        let locations = self.player as *const Player as *const Option<Location>;
        unsafe { *locations.offset(piece.0 as isize) }
    }

    pub fn new(board: &'a Board, pos: &Positions) -> Self {
        let me = board.me();

        let king = me
            .king
            .map(|king| king_moves(pos, king))
            .unwrap_or_default();
        let queen = me
            .queen
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
                .map(|knight| bishop_moves(pos, knight))
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

        let quarters = [
            [king | queen | or(bishop), or(rook) | or(knight)],
            [
                pawn[0] | pawn[1] | pawn[2] | pawn[3],
                pawn[4] | pawn[5] | pawn[6] | pawn[7],
            ],
        ];

        let halves = [
            quarters[0][1] | quarters[1][1],
            quarters[1][0] | quarters[1][0],
        ];

        let all = halves[0] | halves[1];

        MoveCache {
            king,
            queen,
            bishop,
            rook,
            knight,
            pawn,
            all,
            halves,
            quarters,
            player: me,
        }
    }
}
