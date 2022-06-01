#![feature(binary_heap_retain)]
#![doc = include_str!("../readme.md")]
#![allow(dead_code)]
#![warn(unused_crate_dependencies)]
use game::Game;

mod board;
mod cli;
mod game;
mod web; 
mod minimax;
mod moves;
mod opt;
mod parameters;
mod piece;
mod queue;
mod start_board;

fn main() {
    // #[cfg(any(wasm_logger))]
    wasm_logger::init(Default::default());
    log::info!("ASD"); 
    // #[cfg(any(web))]
    web::main();
    
    #[cfg(pretty_env_logger)]
    pretty_env_logger::init();
    #[cfg(cli)]
    cli::main();
}
