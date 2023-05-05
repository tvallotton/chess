use crate::{bishop_moves, location::Location, rook_moves};

use super::Positions;

pub fn queen_moves(pos: &Positions, loc: Location) -> u64 {
    bishop_moves(pos, loc) | rook_moves(pos, loc)
}

#[test]
fn queen_test() {
    use super::utils::debug;
    let mine = (1 << 23) | (1 << 24) | (1 << 55) | (1 << 54) | (1 << 4);
    let opponent = (1 << 8) | (1 << 9) | (1 << 52);

    let pos = Positions::new(mine, opponent);
    let loc = (2, 2).into();
    let moves = queen_moves(&pos, loc);
    println!("mine");
    debug(mine);

    println!("opponent");
    debug(opponent);

    println!("moves");
    debug(moves);
    assert_eq!(moves, 0x40424150e7b0e04);
}
