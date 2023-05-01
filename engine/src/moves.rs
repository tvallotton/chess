use crate::{board::Board, location::Location, piece::Color};

pub struct Move {
    from: Location,
    to: Location,
}

struct Positions {
    white: u64,
    black: u64,
}

pub fn moves(board: Board) -> impl Iterator<Item = Move> {
    None.into_iter()
}

fn positions(board: Board) -> Positions {
    let mut pos = Positions { white: 0, black: 0 };

    for piece in board.pieces {
        let Some((piece, loc)) = piece else {
            continue;
        };
        let table = match piece.color() {
            Color::White => &mut pos.white,
            Color::Black => &mut pos.black,
        };

        *table |= 1 << (loc.rank() * 8 + loc.file());
    }
    pos
}

#[inline]
pub const fn file(n: u64) -> u64 {
    0xff << (8 * n)
}

#[inline]
pub const fn rank(n: u64) -> u64 {
    0x8080808080808080 << n
}

const fn transpose(mut x: u64) -> u64 {
    x = x >> 32 | x << 32;

    let mask: u64 = 0x0000ffff0000ffff;
    x = (x >> 16) & mask | (x & mask) << 16;

    let mask: u64 = 0x00ff00ff00ff00ff;
    x = (x >> 8) & mask | (x & mask) << 8;

    let mask: u64 = 0x0f0f0f0f0f0f0f0f;
    x = (x >> 4) & mask | (x & mask) << 4;

    let mask: u64 = 0x3333333333333333;
    x = (x >> 2) & mask | (x & mask) << 2;

    let mask: u64 = 0x5555555555555555;
    x = (x >> 1) & mask | (x & mask) << 1;

    x
}

fn print(x: u64) {
    for i in 0..8 {
        for j in 0..8 {
            print!("{}", (x >> (i * 8)) >> j & 1);
        }
        println!();
    }
}

#[test]
fn default_positions() {
    print(positions(Default::default()));
}
