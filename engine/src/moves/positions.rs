#![allow(clippy::identity_op)]
use crate::board::Board;

use super::{
    utils::{invert, or},
    Bitfields,
};

pub struct Positions {
    pub opponent: u64,
    pub mine: u64,
    pub mine_inverted: u64,
    pub opponent_inverted: u64,
}

impl Positions {
    pub fn from_board(board: &Board) -> Positions {
        let mine = Bitfields::new(board.me());
        let opponent = Bitfields::new(board.opponent());
        Positions::from_bitfields(mine, opponent)
    }

    pub fn from_bitfields(mine: Bitfields, opponent: Bitfields) -> Positions {
        let mine = 0
            | or(mine.royalty)
            | or(mine.knight)
            | or(mine.pawn)
            | or(mine.rook)
            | or(mine.bishop);

        let opponent = 0
            | or(opponent.royalty)
            | or(opponent.knight)
            | or(opponent.pawn)
            | or(opponent.rook)
            | or(opponent.bishop);

        Positions::new(mine, opponent)
    }

    pub fn new(mine: u64, opponent: u64) -> Positions {
        let mut pos = Positions {
            mine,
            opponent,
            mine_inverted: 0,
            opponent_inverted: 0,
        };
        pos.mine_inverted = invert(pos.mine);
        pos.opponent_inverted = invert(pos.opponent);
        pos
    }

    #[inline]
    pub fn invert(&self) -> Positions {
        Positions {
            opponent: self.opponent_inverted,
            mine: self.mine_inverted,
            opponent_inverted: self.opponent,
            mine_inverted: self.mine,
        }
    }
}
