use std::{fmt::Debug, mem::transmute, num::NonZeroU8, ops::BitOr};

#[derive(Clone, Copy)]

/// Zero is reserved as a niche.
pub struct Piece(pub NonZeroU8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White = 0b10,
    Black = 0b11,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Pawn =   0b00100,
    King =   0b01000,
    Rook =   0b01100,
    Queen =  0b10000,
    Bishop = 0b10100,
    Knight = 0b11000,
}

impl Piece {
    #[inline]
    pub fn kind(self) -> Kind {
        let kind = self.0.get() & 0b11100;
        unsafe { transmute(kind) }
    }
    #[inline]
    pub fn color(self) -> Color {
        let color = self.0.get() & 0b11;
        unsafe { transmute(color) }
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} | {:?}", self.color(), self.kind())
    }
}

impl BitOr<Kind> for Color {
    type Output = Piece;
    fn bitor(self, rhs: Kind) -> Self::Output {
        let kind = rhs as u8;
        let color = self as u8;
        let flags = unsafe { NonZeroU8::new_unchecked(kind | color) };
        Piece(flags)
    }
}
