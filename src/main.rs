#![feature(binary_heap_retain)]
#![doc = include_str!("../readme.md")]
#![allow(dead_code)]
use piece::Color;
use structopt::StructOpt;
use table::Board;
mod minimax;
mod moves;
mod opt;
mod piece;
mod settings;
mod start_board;
mod table;

fn main() {
    let _options = opt::Opt::from_args();

    let node = minimax::MiniMaxNode::default();
    let mut board = node.board;
    let move_ = node.build(); 
    println!("{:?}", move_); 
    board.apply(move_);
    println!("{}", board);
}
