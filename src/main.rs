#![doc = include_str!("../readme.md")]
#![allow(dead_code)]
#![warn(unused_crate_dependencies)]

use std::{
    collections::{HashMap, HashSet},
    default,
    hash::Hash,
};

use board::Board;
use parameters::Params;
use serde_json::from_str;

use crate::piece::Color;

mod board;
mod cli;

mod moves;
mod opt;
mod parameters;
mod piece;
mod queue;
mod start_board;
mod web;

fn main() {
    #[cfg(target_family = "wasm")]
    wasm_logger::init(Default::default());
    #[cfg(target_family = "wasm")]
    web::main();
    // let params = Params::default();
    // let params1 = from_str(include_str!("../params1.json")).unwrap();
    // let params2 = from_str(include_str!("../params2.json")).unwrap();

    // let mut board = Board::default();

    // for _ in 0..200 {
    //     // println!("{board}");
    //     let h = board.heuristic(&params);
    //     if h == f32::INFINITY {
    //         println!("winner: white");
    //         break;
    //     } else if h == f32::NEG_INFINITY {
    //         println!("winner: black");
    //         break;
    //     }
    //     println!("{h}");
    //     let params = match board.turn {
    //         Color::White => &params1,
    //         _ => &params2,
    //     };
    //     if let Some(mv) = board.play_with(params) {
    //         board.apply(mv);
    //         board.advance_turn();
    //     } else {
    //         println!("winner: {}. got none", board.turn.opposite());

    //         break;
    //     }
    // }

    

    // cli::main();
}
