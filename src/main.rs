#![doc = include_str!("../readme.md")]
#![allow(dead_code)]
#![warn(unused_crate_dependencies)]
#![feature(binary_heap_retain)]

use crate::piece::Color;

mod board;
mod cli;
mod heuristic;
mod moves;
mod opt;
mod parameters;
mod piece;
mod piece_tracker;
mod queue;
mod start_board;
mod web;
mod train; 
fn main() {
    #[cfg(target_family = "wasm")]
    wasm_logger::init(Default::default());
    #[cfg(target_family = "wasm")]
    web::main();



    
    

    
}
