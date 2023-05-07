use crate::{location::Location, moves::utils::file};

use super::Positions;

const MASK: u64 = 0xa1100110a; //0x50880088500000;

const LEFT_FILES: u64 = file(0) | file(1);
const RIGHT_FILES: u64 = file(7) | file(6);

pub fn knight_moves(pos: &Positions, loc: Location) -> u64 {
    let up_rank = 8 * loc.rank().saturating_sub(2);
    let down_rank = 8 * 2u8.saturating_sub(loc.rank());
    let right_file = loc.file().saturating_sub(2);
    let left_file = 2u8.saturating_sub(loc.file());
    let moves = MASK << up_rank >> left_file >> down_rank << right_file;
    let f1 = (loc.file() <= 1) as u64 * RIGHT_FILES;
    let f2 = (6 <= loc.file()) as u64 * LEFT_FILES;
    moves & !pos.mine & !f1 & !f2
}

#[test]
fn test_knight() {
    let loc = (6, 6).into();
    assert_eq!(knight_moves(&Positions::new(0, 0), loc), 0x100010a000000000);
}
