use crate::{board::Board, location::Location, piece::Color};

use self::utils::{invert, invert_u64, transpose};

pub mod bishop;
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

pub fn moves(board: Board) -> impl Iterator<Item = Move> {
    None.into_iter()
}

fn positions(board: Board) -> Positions {
    let mut pos = Positions {
        mine: 0,
        opponent: 0,
        mine_inverted: 0,
        opponent_inverted: 0,
    };

    for piece in board.pieces {
        let Some((piece, loc)) = piece else {
            continue;
        };
        let table = if piece.color() == board.meta.turn() {
            &mut pos.mine
        } else {
            &mut pos.opponent
        };

        *table |= 1 << (loc.rank() * 8 + loc.file());
    }
    pos.mine_inverted = invert_u64(pos.mine);
    pos.opponent_inverted = invert_u64(pos.opponent);
    pos
}
impl Positions {
    fn new(mine: u64, opponent: u64) -> Positions {
        let mut pos = Positions {
            mine: 0,
            opponent: 0,
            mine_inverted: 0,
            opponent_inverted: 0,
        };
        pos.mine_inverted = invert_u64(pos.mine);
        pos.opponent_inverted = invert_u64(pos.opponent);

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
