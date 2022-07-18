use crate::board::Props;
use dioxus::prelude::*;
use engine::{Board, Position};

#[derive(Clone, PartialEq)]
pub struct UseSelected {
    pub pos: UseState<Option<Position>>,
    pub allow_play: bool,
    pub board: UseState<Board>,
}
fn use_selected<'a>(s: &'a Scope<Props>) -> UseSelected {
    UseSelected {
        pos: use_state(s, || None).clone(),
        allow_play: s.props.allow_play,
        board: s.props.board.clone(),
    }
}

impl UseSelected {
    pub fn set(&self, pos: Option<Position>) {
        match *self.pos {
            Some(prev) => {
                let mut board: Board = (*self.board).clone();
                for pos in pos {
                    Board::apply(&mut board, (pos, prev).into())
                        .map_err(|_| {
                            self.pos.set(Some(pos));
                        })
                        .ok();
                }
                self.board.set(board);
            }
            None if self.allow_play => {
                self.set_new(pos);
            }
            None => {}
        }
    }

    pub fn set_new(&self, pos: Option<Position>) {
        if let Some(pos) = pos {
            if self.board[pos].is_some() {
                self.pos.set(Some(pos));
            }
        }
    }
}
