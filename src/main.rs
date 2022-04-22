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
    let options = opt::Opt::from_args();

    let board = Board::default();
    let moves: Vec<_> = board
        .moves(Color::White)
        .collect();
    println!("{moves:?}");
    
}
