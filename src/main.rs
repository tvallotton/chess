#![feature(binary_heap_retain)]
#![doc = include_str!("../readme.md")]
#![allow(dead_code)]

use moves::Play;
use piece::Color;

use crate::{board::Board, piece::Kind};
mod board;
mod minimax;
mod moves;
mod opt;
mod piece;
mod queue;
mod settings;
mod start_board;

fn main() {
    pretty_env_logger::init();
    use board::Castle;
    use piece::Color::*;
    use piece::Kind::*;
    use piece::Piece;
    let mut node = minimax::MiniMaxNode::default();
    
    for _ in 0..10 {
        println!("{}", node.board);
        let h = node
            .board
            .heuristic(Color::White);
        println!("board heuristic: {h}");
        println!("node heuristic: {}", node.heuristic);
        // println!("{:?}", node.board);
        node.play();
    }
}

// #[test]
// fn fooooo() {
//     let mut board = Board::empty();
//     board[(4 - 1, 3)] = Some(White | Pawn);
//     board[(5 - 1, 4)] = Some(Black | Pawn);
//     board[(5 - 1, 3)] = Some(White | Pawn);

//     println!("{board}");
// }
