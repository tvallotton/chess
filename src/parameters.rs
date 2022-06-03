use crate::{
    moves::{Move, Position},
    piece::{Color, Kind, Piece},
};

use serde::{Deserialize, Serialize};

use std::{ops::Index, str::FromStr};

use Kind::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Params {
    pub piece_value: f32,
    pub mov_value: f32,
    pub defended: f32,
    pub attacked: f32,
    pub available_moves: f32,
    pub turn_value: f32,
    pub pawn: ValueTable,
    pub king: ValueTable,
    pub queen: ValueTable,
    pub knight: ValueTable,
    pub bishop: ValueTable,
    pub rook: ValueTable,
}

impl Params {
    pub fn piece_val(&self, tuple: (Piece, Position)) -> f32 {
        self.piece_value * self.value(tuple)
    }
    pub fn attacked(&self, attacked: Piece, by: Piece, mov: Move) -> f32 {
        self.attacked * self.value((attacked, mov.to)) / (1.0 + self.value((by, mov.from)))
    }

    pub fn defended(&self, defended: Piece, by: Piece, Move { to, from }: Move) -> f32 {
        self.defended * 1.0 / (1.0 + self.value((by, from)) * self.value((defended, from)).powi(2))
    }
    pub fn mov(&self, piece: Piece, mov: Move) -> f32 {
        self.mov_value * self.value((piece, mov.to))
    }
    pub fn value(&self, tuple: (Piece, Position)) -> f32 {
        let index = (tuple.1, tuple.0.color);
        (match tuple.0.kind {
            Bishop => &self.bishop,
            Rook => &self.rook,
            King => &self.king,
            Knight => &self.knight,
            Queen => &self.queen,
            Pawn => &self.pawn,
        })[index]
    }
}

impl FromStr for Params {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValueTable {
    white: [[f32; 8]; 8],
    black: [[f32; 8]; 8],
}

impl Index<(Position, Color)> for ValueTable {
    type Output = f32;
    fn index(&self, index: (Position, Color)) -> &Self::Output {
        let (pos, color) = index;
        match color {
            Color::White => &self.white[pos.rank as usize][pos.file as usize],
            _ => &self.black[pos.rank as usize][pos.file as usize],
        }
    }
}
