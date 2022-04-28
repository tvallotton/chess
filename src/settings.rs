use crate::{
    moves::Position,
    piece::{Color, Kind, Piece},
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Index;
use tap::prelude::*;

use Kind::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct ValueTable {
    white: [[f32; 8]; 8],
    black: [[f32; 8]; 8],
}
pub fn value(tuple: (Piece, Position)) -> f32 {
    let index = (tuple.1, tuple.0.color);
    (match tuple.0.kind {
        Bishop => &BISHOP,
        Rook => &ROOK,
        King => &KING,
        Knight => &KNIGHT,
        Queen => &QUEEN,
        Pawn => &PAWN,
    })[index]
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
// fn setting<T: Des>(key: &str) -> {

// }

static SETTINGS: Lazy<Value> = Lazy::new(|| {
    let settings = crate::opt::options().settings_path;
    let json = std::fs::read_to_string(settings).unwrap();
    serde_json::from_str(&json).unwrap()
});

pub static PIECE: Lazy<f32> = Lazy::new(|| {
    SETTINGS["piece_value"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});

pub static MAX_NODES: Lazy<usize> = Lazy::new(|| {
    SETTINGS["max_nodes"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});

pub static MAX_ITER: Lazy<usize> = Lazy::new(|| {
    SETTINGS["max_iter"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});
pub static DEFENDED: Lazy<f32> = Lazy::new(|| {
    SETTINGS["defended_value"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});

pub static ATTACKED: Lazy<f32> = Lazy::new(|| {
    SETTINGS["attacked_value"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});

pub static AVAILABLE_MOVES: Lazy<f32> = Lazy::new(|| {
    SETTINGS["available_moves"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});

pub static PAWN: Lazy<ValueTable> = Lazy::new(|| {
    SETTINGS["pawn"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});
pub static KING: Lazy<ValueTable> = Lazy::new(|| {
    SETTINGS["king"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});
pub static QUEEN: Lazy<ValueTable> = Lazy::new(|| {
    SETTINGS["queen"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});
pub static BISHOP: Lazy<ValueTable> = Lazy::new(|| {
    SETTINGS["bishop"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});
pub static KNIGHT: Lazy<ValueTable> = Lazy::new(|| {
    SETTINGS["knight"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});
pub static ROOK: Lazy<ValueTable> = Lazy::new(|| {
    SETTINGS["rook"]
        .clone()
        .pipe(serde_json::from_value)
        .unwrap()
});
