#![allow(clippy::precedence)]
use crate::location::Location;

use super::{utils::debug, Positions};

pub(super) fn bishop_moves(pos: Positions, loc: Location) -> u64 {
    todo!()
}

pub(super) fn two_diagonals(pos: &Positions, loc: Location) -> u64 {
    let first = compute_single(&pos, loc);
    let second = compute_single(&pos.invert(), loc.invert());

    first | second
}

/// Computes a single stoke in the X formed by the bishop
pub(super) fn compute_single(pos: &Positions, loc: Location) -> u64 {
    let rank = loc.rank();
    let diag = diagonal(loc);

    // 1. We discard the bits before the input so they don't interfer
    let diag = diag >> 8 * rank << 8 * rank;

    // 2. We only keep the positions on the diagonal
    let mine = pos.mine & diag;
    let opponent = pos.opponent & diag;

    // 3. we add to max, so any overflowing bits get removed (blocking the bishop)
    let unblocked_m = mine
        .overflowing_add(u64::MAX)
        .0;

    // 4. The opponent can be catured, so it is first moved one rank
    let unblocked_o = (opponent << 8)
        .overflowing_add(u64::MAX)
        .0;

    unblocked_m & unblocked_o & diag & !loc.pos()
}

const DIAG: [u64; 2] = [0x8040201008040201, 0x102040810204080];

#[inline]
fn diagonal(loc: Location) -> u64 {
    let rank = loc.rank();
    let file = loc.file();
    // DIAG[0] << 8 * rank.saturating_sub(file) >> 8 * file.saturating_sub(rank),
    // DIAG[1] >> 8 * 7u8.saturating_sub(rank + file) << 8 * (rank + file).saturating_sub(7),
    DIAG[0] << 8 * rank.saturating_sub(file) >> 8 * file.saturating_sub(rank)
}

#[test]
fn test_diag() {
    let mine = (1 << 8) | (1 << 9);
    let opponent = (1 << 55) | (1 << 54);
    let pos = Positions::new(mine, opponent);
    debug(mine);
    debug(opponent);
    debug(compute_single(&pos, (2, 2).into()));
    debug(Location::from((1, 4)).pos());
}
