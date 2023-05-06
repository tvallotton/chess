use std::{fmt::Debug, mem::transmute, num::NonZeroU8, ops::BitOr};

#[derive(Clone, Copy)]

/// Zero is reserved as a niche.
pub struct Piece(pub NonZeroU8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White = 0b10,
    Black = 0b11,
}

pub const PAWN: u8 = 0b00100;
pub const BISHOP: u8 = 0b01000;
pub const KNIGHT: u8 = 0b01100;
pub const QUEEN: u8 = 0b10000;
pub const KING: u8 = 0b10100;
pub const ROOK: u8 = 0b11000;
