use std::iter::from_fn;

use crate::{board::Board, location::Location};

use self::utils::invert;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;
mod utils;

pub struct Move {
    from: Location,
    to: Location,
}

pub struct Positions {
    opponent: u64,
    mine: u64,
    mine_inverted: u64,
    opponent_inverted: u64,
}

pub fn moves(_board: Board) -> impl Iterator<Item = Move> {
    None.into_iter()
}

pub fn bitfields(board: Board) {
    todo!()
}

fn positions(board: Board) -> Positions {
    todo!()
}
impl Positions {
    fn new(mine: u64, opponent: u64) -> Positions {
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
    fn invert(&self) -> Positions {
        Positions {
            opponent: self.opponent_inverted,
            mine: self.mine_inverted,
            opponent_inverted: self.opponent,
            mine_inverted: self.mine,
        }
    }
}

#[test]
fn default_positions() {
    utils::debug(positions(Default::default()).mine);
}
