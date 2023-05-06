use crate::{
    board::{Board, Player},
    location::Location,
};

use super::{utils::invert, Bitfields};

pub struct Positions {
    pub opponent: u64,
    pub mine: u64,
    pub mine_inverted: u64,
    pub opponent_inverted: u64,
}

impl Positions {
    pub fn from_board(mine: Bitfields, opponent: Bitfields) -> Positions {
        fn or<const D: usize>(array: [u64; D]) -> u64 {
            array
                .iter()
                .fold(0, |x, y| x | y)
        }

        let mine = mine.king
            | mine.queen
            | or(mine.knight)
            | or(mine.pawn)
            | or(mine.rook)
            | or(mine.bishop);

        let opponent = opponent.king
            | opponent.queen
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
