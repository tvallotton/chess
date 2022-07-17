use crate::square::Square;
use dioxus::prelude::*;
use engine::{Board, Color, Move, Position};
use std::rc::Rc;

#[derive(Props, PartialEq)]
pub struct Props {

}


struct UseSelected; 
impl UseSelected {
    fn set(mut board: Board, prev: Position, pos: Position) {
        board.apply(Move::from((prev, pos)));
    }
}
