use std::{fmt::Debug, mem::transmute};

use crate::piece::Color;
use masks::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Metadata(u8, bool);

#[rustfmt::skip]
mod masks {
   pub const CURRENT_TURN: u8      = 0b00000001;
   pub const KINGSIDE_CASTLE: u8   = 0b00000110;
   pub const QUEENSIDE_CASTLE: u8  = 0b00011000;
   pub const PASSANT_PAWN_FILE: u8 = 0b11100000;
}

impl Metadata {
    pub fn turn(self) -> Color {
        unsafe { transmute(self.0 & CURRENT_TURN << 1) }
    }

    pub fn kingside_castle(self, color: Color) -> bool {
        let castles = self.0 & KINGSIDE_CASTLE;
        (castles & color as u8) != 0
    }

    pub fn queenside_castle(self, color: Color) -> bool {
        let castles = (self.0 & QUEENSIDE_CASTLE) >> 2;
        (castles & color as u8) != 0
    }

    pub fn lose_kingside(&mut self, color: Color) {
        self.0 &= !(self.0 & KINGSIDE_CASTLE & color as u8);
    }

    pub fn lose_queenside(&mut self, color: Color) {
        let bit = (color as u8) << 2;
        self.0 &= !(self.0 & QUEENSIDE_CASTLE & bit);
    }
    pub fn set_passant(&mut self, passant: Option<u8>) {
        if let Some(passant) = passant {
            let _file = (passant << 5) & PASSANT_PAWN_FILE;
            self.1 = true;
        } else {
            self.1 = false
        }
    }
    pub fn passant_file(self) -> Option<u8> {
        if self.1 {
            let file = (self.0 & PASSANT_PAWN_FILE) >> 5;
            Some(file)
        } else {
            None
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata(0b00011111, false)
    }
}

impl Debug for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Metadata")
            .field("kingside_castle_white", &self.kingside_castle(Color::Black))
            .field("kingside_castle_black", &self.kingside_castle(Color::White))
            .field(
                "queenside_castle_white",
                &self.queenside_castle(Color::Black),
            )
            .field(
                "queenside_castle_black",
                &self.queenside_castle(Color::White),
            )
            .finish()
    }
}
