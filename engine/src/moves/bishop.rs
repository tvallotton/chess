#![allow(clippy::precedence)]
use crate::location::Location;

use super::{utils::debug, Positions};

pub(super) fn bishop_moves(pos: Positions, loc: Location) -> u64 {
    todo!()
}

const DIAG: [u64; 2] = [0x8040201008040201, 0x102040810204080];

fn diagonal(loc: Location) -> [u64; 2] {
    let rank = loc.rank();
    let file = loc.file();

    [
        DIAG[0] << 8 * rank.saturating_sub(file) >> 8 * file.saturating_sub(rank),
        DIAG[1] >> 8 * 7u8.saturating_sub(rank + file) << 8 * (rank + file).saturating_sub(7),
    ]
}

#[test]
fn test_bishop() {
    // good
    let pos = (4, 5).into();
    let [x, y] = diagonal(pos);
    debug((x | y) & !pos.pos());
    // bad
    let pos = (5, 2).into();
    let [x, y] = diagonal(pos);
    debug((x | y) & !pos.pos());
}
