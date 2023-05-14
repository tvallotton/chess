use crate::{board::Player, location::Location, moves::Positions};

use super::{utils::file, Bitfields};

const MASK: u64 = 0b000001110000010100000111;

pub fn king_moves(pos: &Positions, loc: Location) -> u64 {
    let up_rank = 8 * loc.rank().saturating_sub(1);
    let down_rank = 8 * (loc.rank() == 0) as u8;
    let right_file = loc.file().saturating_sub(1);
    let left_file = (loc.file() == 0) as u8;
    let moves = MASK << up_rank >> left_file >> down_rank << right_file;
    let f1 = (loc.file() == 0) as u64 * file(7);
    let f2 = (loc.file() == 7) as u64 * file(0);
    (moves & !pos.mine) & !f1 & !f2
}

pub fn is_check(_: Bitfields) {
    todo!()
}

#[test]
fn test_king() {
    use super::utils::debug;
    let loc = (3, 0).into();
    debug(king_moves(&Positions::new(0, 0), loc));
    debug(loc.pos());
}
