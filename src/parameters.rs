use crate::{
    moves::{Move, Position},
    opt::Settings,
    piece::{Color, Kind, Piece},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tap::Pipe;

use std::{ops::Index, str::FromStr};

use Kind::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Params {
    pub sort_depth: i32,
    pub presort_depth: i32,
    pub max_iter: i32,
    pub piece_value: f32,
    pub mov_value: f32,
    pub defended: f32,
    pub attacked: f32,
    pub available_moves: f32,
    pub castle_kingside: f32,
    pub castle_queenside: f32,
    pub material_only: bool,
    pub max_depth: usize,
    pub pawn: ValueTable,
    pub king: ValueTable,
    pub queen: ValueTable,
    pub knight: ValueTable,
    pub bishop: ValueTable,
    pub rook: ValueTable,
    pub black_algorithm: Algorithm,
    pub white_algorithm: Algorithm,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Algorithm {
    AlphaBetaPruning,
    Prepruning,
}
fn random() -> f32 {
    rand::random()
}
const LEARNING_RATE: f32 = 1.0;
impl Params {
    pub fn split(&self) -> (Params, Params) {
        let mut pos = self.clone();
        let mut neg = self.clone();

        let r = random();
        pos.attacked += r * LEARNING_RATE;
        neg.attacked -= r * LEARNING_RATE;

        let r = random();
        pos.defended += r * LEARNING_RATE;
        neg.defended -= r * LEARNING_RATE;

        let r = random();
        pos.castle_kingside += r * LEARNING_RATE;
        neg.castle_kingside -= r * LEARNING_RATE;

        let r = random();
        pos.castle_queenside += r * LEARNING_RATE;
        neg.castle_queenside -= r * LEARNING_RATE;

        let r = random();
        pos.available_moves += r * LEARNING_RATE;
        neg.available_moves -= r * LEARNING_RATE;

        let r = random();
        pos.mov_value += r * LEARNING_RATE;
        neg.mov_value -= r * LEARNING_RATE;

        (neg, pos)
    }

    pub fn algorithm(&self, color: Color) -> Algorithm {
        match color {
            Color::Black => self.black_algorithm,
            Color::White => self.white_algorithm,
        }
    }

    pub fn piece_value(&self, piece: (Piece, Position)) -> f32 {
        self.piece_value * self.value(piece)
    }

    pub fn attacked(&self, attacked: Piece, by: Piece, mov: Move) -> f32 {
        self.attacked * 0.0f32.max(self.value((attacked, mov.to)) - self.value((by, mov.from)))
    }

    pub fn defended(&self, defended: Piece, by: Piece, Move { to: _, from }: Move) -> f32 {
        self.defended * 0.0f32.max(self.value((by, from)) - self.value((defended, from)))
    }
    pub fn mov(&self, piece: Piece, mov: Move) -> f32 {
        self.mov_value / (self.value((piece, mov.to)) + 1.0)
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

impl Default for Params {
    fn default() -> Self {
        // let settings: Settings = include_str!("../settings.json")
        //     .pipe(serde_json::from_str)
        //     .unwrap();
        // settings.absolute_params
        include_str!("../params.json")
            .pipe(serde_json::from_str)
            .unwrap()
    }
}

impl FromStr for Params {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValueTable([[f32; 8]; 8]);

impl ValueTable {
    fn split(&self) -> (Self, Self) {
        let mut first = self.clone();
        let mut second = self.clone();

        for i in 0..8 {
            for j in 0..8 {
                let r = rand::random::<f32>() * 0.1;
                first.0[i][j] += r;
                second.0[i][j] -= r;
            }
        }
        (first, second)
    }
}

impl Index<(Position, Color)> for ValueTable {
    type Output = f32;
    fn index(&self, index: (Position, Color)) -> &Self::Output {
        let (pos, color) = index;
        let rank = match color {
            Color::White => pos.rank,
            _ => 7 - pos.rank,
        } as usize;
        &self.0[rank][pos.file as usize]
    }
}
