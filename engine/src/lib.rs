#![allow(dead_code)]
#![warn(unused_crate_dependencies)]
#![feature(binary_heap_retain)]



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

mod train;

pub use board::Board;
pub use moves::{Position, Move}; 
pub use piece::{Color, Piece, Kind}; 