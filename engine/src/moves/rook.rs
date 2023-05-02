#![allow(arithmetic_overflow)]

use crate::{location::Location, piece::Color};

use super::{
    utils::{debug, file, invert, invert_u64, rank, transpose},
    Move, Positions,
};

pub(super) fn rook_moves(pos: &Positions, loc: Location) {}

pub(super) fn rook_positions(pos: &Positions, loc: Location) -> u64 {
    let rank = rank_positions(pos, loc);
    let file = rank_positions(&pos.transpose(), loc.transpose());

    (rank | transpose(file)) & !loc.pos()
}

pub(super) fn rank_positions(pos: &Positions, loc: Location) -> u64 {
    let leftside = rank_leftside(pos, loc);
    let rightside = invert_u64(rank_leftside(&pos.invert(), loc.invert()));
    leftside | rightside
}

/// Computes valid bits to the left to the rook. Right hand side bits are
/// not considered well defined.
pub(super) fn rank_leftside(pos: &Positions, loc: Location) -> u64 {
    let brank = rank(loc.rank());

    // 1. We discard the rightside bits of the rank
    // so they don't interfere
    //
    // 2. We add the brank with mine to carry out the
    // bits after the blocking piece. E.g.
    // ```
    // 0b11111111 + 0b01001000 = 0b101000111
    // ```
    //
    // 3. We ingore carries outside the rank with `& brank`
    // ```
    // 0b101000111 & 0b11111111 = 0b01000111
    //   ^ we want to remove this bit
    // ```
    // 4. We remove pieces in the blocked section if any `& !pos.mine`
    // ```
    // 0b01000111 & !0b01001000 = 0b00000111
    //                  ^ we want to remove this bit
    // ```
    let ignore = brank + loc.pos();
    let brank = brank & !ignore;

    let self_block = (pos.mine + brank) & brank & !pos.mine;

    // We do the same as above but with oponent shifted to the left by one bit
    // because this will allow us to include it in the set of available moves
    let attack = (2 * pos.opponent + brank) & brank & !(2 * pos.opponent);

    // We intersect both sets to find the answer
    self_block & attack
}

#[test]
fn rank_test() {
    let mine = (1 << 15) | (1 << 14);
    let opponent = (1 << 8) | (1 << 9);
    let pos = Positions {
        opponent,
        mine_inverted: invert_u64(mine),
        mine,
        opponent_inverted: invert_u64(opponent),
        mine_transposed: mine,
        opponent_transposed: mine,
    };

    assert_eq!(rank_positions(&pos, (1, 4).into()), 0b00101110 << 8);
}

#[test]
fn rook_test() {
    let mine = (1 << 15) | (1 << 14);
    let opponent = (1 << 8) | (1 << 9);
    let pos = Positions {
        opponent,
        mine_inverted: invert_u64(mine),
        mine,
        opponent_inverted: invert_u64(opponent),
        mine_transposed: mine,
        opponent_transposed: mine,
    };

    assert_eq!(rook_positions(&pos, (1, 4).into()), 1157442765409234448);
}
