use crate::{
    bishop_moves, board::Board, king_moves, location::Location, moves::bishop, pawn_moves,
    queen_moves, rook_moves,
};

use super::{utils::or, Positions};

pub struct MoveCache {
    king: u64,
    queen: u64,
    bishop: [u64; 2],
    rook: [u64; 2],
    knight: [u64; 2],
    pawn: [u64; 8],
    union: u64,
}

impl MoveCache {
    pub fn new(board: Board, pos: &Positions) -> Self {
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

        // const PAWN_MOVES: [fn(&Positions, Location) -> u64; 2] = [queen_moves, pawn_moves];

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

        let union = king | queen | or(rook) | or(bishop) | or(knight) | or(pawn);

        MoveCache {
            king,
            queen,
            bishop,
            rook,
            knight,
            pawn,
            union,
        }
    }
}
