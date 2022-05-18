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
    
    let mut game = Game::new();
    
    for _i in 0..50 {
        println!("{game}"); 
        if !game.play() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100)); 
    }

    game.winner();
}

fn play_game() {}


