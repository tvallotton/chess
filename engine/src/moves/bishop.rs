use crate::location::Location;

use super::{utils::debug, Positions};

pub(super) fn bishop_moves(pos: Positions, loc: Location) -> u64 {
    todo!()
}

const MASK_TOP_RIGHT: u64 = 0x8040201008040201;
const MASK_TOP_LEFT: u64 = 0x102040810204080;

fn up_leftside_diag(pos: Positions, loc: Location) -> u64 {
    let rank = loc.rank();
    let file = loc.file();

    todo!()
}

fn diagonal(loc: Location) -> [u64; 2] {
    let rank = loc.rank();
    let file = loc.file();
    let rightshift = 8 * 6u8.saturating_sub(rank + file);
    let leftshift = 8 * (rank + file).saturating_sub(7);
    [
        MASK_TOP_RIGHT << leftshift >> rightshift,
        MASK_TOP_LEFT << leftshift >> rightshift,
    ]
}

#[test]
fn test_bishop() {
    // good
    let pos = (2, 3).into();
    let [x, _] = diagonal(pos);
    debug(x & !pos.pos());
    // bad
    let pos = (4, 5).into();
    let [x, _] = diagonal(pos);
    debug(x & !pos.pos());
    // good
    let pos = (4, 5).into();
    let [_, x] = diagonal(pos);
    debug(x & !pos.pos());
    // bad
    let pos = (2, 3).into();
    let [_, x] = diagonal(pos);
    debug(x & !pos.pos());
}
