use std::ops::BitOr;

use serde::de::IntoDeserializer;

use crate::piece::Piece;

#[derive(Clone, Copy)]
pub struct Location(u8);

impl Location {
    pub fn rank(self) -> u8 {
        (self.0 >> 3) & 0b111
    }
    pub fn file(self) -> u8 {
        self.0 & 0b111
    }

    pub fn pos(self) -> u64 {
        1 << (self.rank() * 8 + self.file())
    }

    pub fn invert(self) -> Self {
        (7 - self.rank(), 7 - self.file()).into()
    }

    pub fn transpose(self) -> Location {
        (self.file(), self.rank()).into()
    }
}

pub fn loc(rank: u8, file: u8) -> Location {
    (rank, file).into()
}

impl<T: Into<Location>> BitOr<T> for Piece {
    type Output = (Piece, Location);
    fn bitor(self, rhs: T) -> Self::Output {
        (self, rhs.into())
    }
}

impl From<(u8, u8)> for Location {
    fn from((rank, file): (u8, u8)) -> Self {
        Location((rank << 3 | file) as u8)
    }
}