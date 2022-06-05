#![doc = include_str!("../readme.md")]
#![allow(dead_code)]
#![warn(unused_crate_dependencies)]


mod board;
mod cli;

mod moves;
mod opt;
mod parameters;
mod piece;
mod queue;
mod start_board;
mod web;

fn main() {
    #[cfg(target_family = "wasm")]
    wasm_logger::init(Default::default());
    log::info!("wasm");
    #[cfg(target_family = "wasm")]
    web::main();
    
    #[cfg(not(target_family = "wasm"))]
    pretty_env_logger::init();
    
    // cli::main();
}
