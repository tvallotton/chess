#![allow(non_upper_case_globals)]
#![allow(dead_code, unused)]
#[macro_use]
extern crate dioxus_html_macro;



// mod board;


use dioxus::prelude::*;
use engine::{Board, Color, Move, Position};


struct UseSelected; 
impl UseSelected {
    fn set(mut board: Board, prev: Position, pos: Position) {
        board.apply(Move::from((prev, pos)));
    }
}

fn main() {}
