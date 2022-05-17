#![feature(binary_heap_retain)]
#![doc = include_str!("../readme.md")]
#![allow(dead_code)]

use game::Game;


mod board;
mod game;
mod minimax;
mod moves;
mod opt;
mod parameters;
mod piece;
mod queue;
mod start_board;

fn main() {
    pretty_env_logger::init();
    print!("asd\n"); 
    let mut game = Game::new();
    print!("asd\n"); 
    for _i in 0..20 {
        println!("{}", game);
        if !game.play() {
            break;
        }
    }

    game.winner();
}

fn play_game() {}
