use crate::{
    moves::{Move, Position},
    piece::{Color, Kind, Piece},
};

use serde::{Deserialize, Serialize};

use std::{ops::Index, str::FromStr};

use Kind::*;

#[derive(Debug, Deserialize)]
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
    pub fn attacked(&self, attacked: (Piece, Position), by: (Piece, Position)) -> f32 {
        self.attacked * self.value(attacked) / self.value(by)
    }
    pub fn defended(&self, defended: (Piece, Position), by: (Piece, Position)) -> f32 {
        self.defended * self.value(by) / self.value(defended)
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

#[derive(Debug, Serialize, Deserialize)]
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
// fn setting<T: Des>(key: &str) -> {

// }

// static SETTINGS: Lazy<Value> = Lazy::new(|| {
//     let settings = crate::opt::options().settings_path;
//     let json = std::fs::read_to_string(settings).unwrap();
//     serde_json::from_str(&json).unwrap()
// });

// pub static PIECE: Lazy<f32> = Lazy::new(|| {
//     SETTINGS["piece_value"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });

// pub static MAX_NODES: Lazy<usize> = Lazy::new(|| {
//     SETTINGS["max_nodes"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });

// pub static MAX_ITER: Lazy<usize> = Lazy::new(|| {
//     SETTINGS["max_iter"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });
// pub static DEFENDED: Lazy<f32> = Lazy::new(|| {
//     SETTINGS["defended_value"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });

// pub static ATTACKED: Lazy<f32> = Lazy::new(|| {
//     SETTINGS["attacked_value"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });

// pub static AVAILABLE_MOVES: Lazy<f32> = Lazy::new(|| {
//     SETTINGS["available_moves"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });

// pub static PAWN: Lazy<ValueTable> = Lazy::new(|| {
//     SETTINGS["pawn"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });
// pub static KING: Lazy<ValueTable> = Lazy::new(|| {
//     SETTINGS["king"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });
// pub static QUEEN: Lazy<ValueTable> = Lazy::new(|| {
//     SETTINGS["queen"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });
// pub static BISHOP: Lazy<ValueTable> = Lazy::new(|| {
//     SETTINGS["bishop"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });
// pub static KNIGHT: Lazy<ValueTable> = Lazy::new(|| {
//     SETTINGS["knight"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });
// pub static ROOK: Lazy<ValueTable> = Lazy::new(|| {
//     SETTINGS["rook"]
//         .clone()
//         .pipe(serde_json::from_value)
//         .unwrap()
// });
