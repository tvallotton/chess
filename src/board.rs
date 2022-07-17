use crate::square::Square;
use dioxus::prelude::*;
use engine::{Board, Color, Position, Move};
use std::rc::Rc;

#[derive(Props, PartialEq)]
pub struct Props {
    board: UseState<Board>,
    selected: UseState<Option<Position>>,
    play_as: Color,
    allow_play: bool,
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
    let board = &s.props.board;
    let mut total = html!();
    let play_as = s.props.play_as;
    let ref range = range(play_as);
    let highlighted = Rc::new(highlighted(s));
    let selected = use_selected(s);

    for &rank in range {
        let mut row = html!();
        for &file in range {
            let color = color(rank, file);
            let piece = board[(rank, file)];
            let highlighted = highlighted.clone();
            let square = html! (
                <Square
                    color={color}
                    piece={piece}
                    highlighted={highlighted}
                    pos={(rank, file).into()}
                    selected={props.selected.clone()}/>
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

fn highlighted(s: &Scope<Props>) -> Vec<Position> {
    let props = s.props;

    for &selected in props.selected.get() {
        if props.board[selected].is_some() {
            return props
                .board
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

struct UseSelected {
    pos: UseState<Option<Position>>,
    allow_play: bool,
    board: UseState<Board>,
}

impl UseSelected {
    fn set(&self, pos: Option<Position>) {
        match *self.pos {
            Some(prev) => {
                let mut board: Board = (*self.board).clone();
                for pos in pos {
                    board.apply(Move::from((prev, pos)));
                }
                self.board.set(board);
            }
            None if self.allow_play => {
                self.pos.set(pos);
            }
            None => {}
        }
    }
}

fn range(play_as: Color) -> Vec<i8> {
    match play_as {
        Color::White => (0..8).collect(),
        Color::Black => (0..8).rev().collect(),
    }
}
