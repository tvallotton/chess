use super::{
    utils::{debug, file},
    Positions,
};
use crate::{location::Location, moves::utils::rank, piece::Color};

pub fn pawn_moves(pos: &Positions, loc: Location, color: Color) -> u64 {
    let captures = left_captures(pos, loc, color) | right_captures(pos, loc, color);
    (captures & pos.opponent) | forwards(pos, loc, color)
}

#[inline]
pub fn left_captures(pos: &Positions, loc: Location, color: Color) -> u64 {
    let white = (color == Color::White) as u8;
    let black = (color == Color::Black) as u8;
    loc.pos() >> (9 * white) << (9 * black) & !file(7)
}

#[inline]
pub fn right_captures(pos: &Positions, loc: Location, color: Color) -> u64 {
    let white = (color == Color::White) as u8;
    let black = (color == Color::Black) as u8;
    loc.pos() >> (7 * white) << (7 * black) & !file(0)
}
#[inline]
pub fn forwards(pos: &Positions, loc: Location, color: Color) -> u64 {
    let white = (color == Color::White) as u64;
    let black = (color == Color::Black) as u64;
    let one = (loc.pos() >> (8 * white) << (8 * black)) & !pos.mine & !pos.opponent;
    let two =
        ((one & (rank(5) | rank(2))) >> (8 * white) << (8 * black)) & !pos.mine & !pos.opponent;

    one | two
}

#[test]
fn test_pawns() {
    let opponent = Location::from((4, 5));

    assert_eq!(
        pawn_moves(
            &Positions::new(0, opponent.pos()),
            (5, 4).into(),
            Color::White,
        ),
        206158430208
    );
    assert_eq!(
        pawn_moves(&Positions::new(0, 0), (5, 4).into(), Color::White,),
        68719476736
    );

    assert_eq!(
        pawn_moves(&Positions::new(0, 1 << 21), (1, 4).into(), Color::Black,),
        271581184
    );

    assert_eq!(
        pawn_moves(&Positions::new(0, 1 << 20), (1, 4).into(), Color::Black,),
        0
    );
}
