#![feature(binary_heap_retain)]
#![doc = include_str!("../readme.md")]
#![allow(dead_code)]

use game::Game;

mod board;
mod cli;
#[cfg(structopt)]
mod cli;
mod game;
#[cfg(yew)]
mod gui;
mod minimax;
mod moves;
mod opt;
mod parameters;
mod piece;
mod queue;
mod start_board;

#[cfg(target_family = "wasm")]
fn wasm() {}

fn main() {
    #[cfg(wasm)]
    {
        wasm_logger::init(Default::default());
        gui::main();
    }
    #[cfg(not(wasm))]
    {
        pretty_env_logger::init();
        cli::main();
    }
}
