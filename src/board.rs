use crate::square::Square;
use dioxus::prelude::*;
use engine::{Board, Color, Move, Position};
use std::rc::Rc;

#[derive(Props, PartialEq)]
pub struct Props {
    pub board: UseState<Board>,
    pub selected: UseState<Option<Position>>,
    pub play_as: Color,

    pub allow_play: bool,
}

pub fn color(rank: i8, file: i8) -> Color {
    if (rank + file) % 2 == 0 {
        Color::White
    } else {
        Color::Black
    }
}

pub const Board: Component<Props> = |ref s| {
    let props = s.props;
    let board = &props.board;
    let mut total = html!();
    let play_as = s.props.play_as;
    let ref range = range(play_as);
    let selected = use_selected(s);
    let highlighted = Rc::new(highlighted(board, &selected));

    for &rank in range {
        let mut row = html!();
        for &file in range {
            let color = color(rank, file);
            let piece = board[(rank, file)];
            let highlighted = highlighted.clone();
            let selected = selected.clone();
            let square = html! (
                <Square
                    color={color}
                    piece={piece}
                    highlighted={highlighted}
                    pos={(rank, file).into()}
                    selected={selected.clone()}/>
            );
            row = html!(
                {row} {square}
            );
        }
        total = html!(
            {total}
            <div display="flex">
                {row}
            </div>
        );
    }

    s.render(total)
};

fn highlighted(board: &Board, selected: &UseSelected) -> Vec<Position> {
    for &selected in &*selected.pos {
        if board[selected].is_some() {
            return board
                .moves_for_piece(selected.into())
                .map(|mv| mv.to)
                .collect();
        }
    }
    vec![]
}

fn use_selected<'a>(s: &'a Scope<Props>) -> UseSelected {
    UseSelected {
        pos: use_state(s, || None).clone(),
        allow_play: s.props.allow_play,
        board: s.props.board.clone(),
    }
}
#[derive(Clone, PartialEq)]
pub struct UseSelected {
    pub pos: UseState<Option<Position>>,
    pub allow_play: bool,
    pub board: UseState<Board>,
}

impl UseSelected {
    pub fn set(&self, pos: Position) {
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
    pub fn is_valid(&self, pos: Position) -> bool {
        if let Some(piece) = self.board[pos] {
            log::error!(
                "{} && {} == {}",
                self.allow_play,
                piece.color,
                self.board.turn
            );
            return self.allow_play && piece.color == self.board.turn;
        }
        false
    }

    pub fn set_new(&self, pos: Position) {
        if self.is_valid(pos) {
            self.pos.set(Some(pos));
        }
    }
}

fn range(play_as: Color) -> Vec<i8> {
    match play_as {
        Color::White => (0..8).collect(),
        Color::Black => (0..8).rev().collect(),
    }
}
