use crate::board::Props;
use dioxus::prelude::*;
use engine::{Board, Location};

#[derive(Clone, PartialEq)]
pub struct UseSelected {
    pub pos: UseState<Option<Location>>,
    pub allow_play: bool,
    pub board: UseState<Board>,
}

pub fn use_selected<'a>(s: &'a Scope<Props>) -> UseSelected {
    UseSelected {
        pos: use_state(s, || None).clone(),
        allow_play: s.props.allow_play,
        board: s.props.board.clone(),
    }
}

impl UseSelected {
    pub fn set(&self, pos: Location) {
        match *self.pos {
            Some(prev) => {
                let mut board: Board = (*self.board).clone();
                if let Ok(_) = (&mut board).apply((pos, prev).into()) {
                    self.board.set(board);
                    self.pos.set(None);
                } else {
                    self.pos.set(None);
                    self.set_new(pos);
                }
            }
            None => self.set_new(pos),
        }
    }
    pub fn is_valid(&self, pos: Location) -> bool {
        if let Some(piece) = self.board[pos] {
            return self.allow_play && piece.color == self.board.turn;
        }
        false
    }

    pub fn set_new(&self, pos: Location) {
        if self.is_valid(pos) {
            self.pos.set(Some(pos));
        }
    }
}
