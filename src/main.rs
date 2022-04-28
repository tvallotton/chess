#![feature(binary_heap_retain)]
#![doc = include_str!("../readme.md")]
#![allow(dead_code)]

use moves::Play;
use piece::Color;
use serde_json::from_value;
use structopt::StructOpt;

use crate::{board::Board, piece::Kind, opt::Opt};
mod board;
mod minimax;
mod moves;
mod opt;
mod piece;
mod queue;
mod parameters;
mod start_board;
mod game; 

fn main() {
    pretty_env_logger::init();
    let _opt = opt::Opt::from_args(); 

    
    // return; 
    // use board::Castle;
    // use piece::Color::*;
    // use piece::Kind::*;
    // use piece::Piece;
    // let mut node = minimax::MiniMaxNode::default();
    
    // for _ in 0..10 {
    //     println!("{}", node.board);
    //     let h = node
    //         .board
    //         .heuristic(Color::White, );
    //     println!("board heuristic: {h}");
    //     println!("node heuristic: {}", node.heuristic);
    //     // println!("{:?}", node.board);
    //     node.play();
    // }
}



fn play_game() {

    

}