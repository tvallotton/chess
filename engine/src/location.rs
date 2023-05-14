use std::{fmt::Debug, num::NonZeroU8, ops::BitOr};

use crate::piece::PieceIndex;

#[derive(Clone, Copy)]
pub struct Location(NonZeroU8);

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        (self.0.get() & 0b01111110) == (other.0.get() & 0b01111110)
    }
}

impl Location {
    pub fn rank(self) -> u8 {
        (self.0.get() >> 4) & 0b111
    }
    pub fn file(self) -> u8 {
        (self.0.get() >> 1) & 0b111
    }

    pub fn pos(self) -> u64 {
        1 << (self.rank() * 8 + self.file())
    }

    pub fn from_pos(bit: u64) -> Self {
        let log = bit
            .checked_ilog2()
            .unwrap_or_default();
        let rank = log / 8;
        let file = log % 8;
        loc(rank as u8, file as u8)
    }
    #[inline]
    pub fn is_queen(self) -> bool {
        (self.0.get() & 0b10000000) != 0
    }
    #[inline]
    pub fn promote(&mut self) {
        self.0 |= 0b10000000
    }

    pub fn set(&mut self, loc: Location) {
        self.0 = unsafe {
            NonZeroU8::new_unchecked(loc.0.get() & 0b01111111) | (self.0.get() & 0b10000000)
        };
    }

    #[inline]
    pub fn invert(self) -> Self {
        (7 - self.rank(), 7 - self.file()).into()
    }
    #[inline]
    pub fn transpose(self) -> Location {
        (self.file(), self.rank()).into()
    }
}

pub fn loc(rank: u8, file: u8) -> Location {
    (rank, file).into()
}

impl<T: Into<Location>> BitOr<T> for PieceIndex {
    type Output = (PieceIndex, Location);
    fn bitor(self, rhs: T) -> Self::Output {
        (self, rhs.into())
    }
}

impl From<(u8, u8)> for Location {
    fn from((rank, file): (u8, u8)) -> Self {
        let non_zero = ((rank << 3) | file) << 1 | 1;
        unsafe { Location(NonZeroU8::new_unchecked(non_zero)) }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.rank(), self.file())
    }
}

#[test]
fn from_pos() {
    for rank in 0..8 {
        for file in 0..8 {
            let pos = loc(rank, file).pos();
            assert_eq!(pos, Location::from_pos(pos).pos())
        }
    }
}
