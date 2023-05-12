use crate::square::Square;
use crate::use_select::{use_selected, UseSelected};
use dioxus::prelude::*;
use engine::moves;
use engine::{Board, Color, Location, Move, Params};
use std::rc::Rc;

#[derive(Props, PartialEq)]
pub struct Props {
    pub board: UseState<Board>,
    pub selected: UseState<Option<Location>>,
    pub play_as: Color,
    pub params: Params,
    pub allow_play: bool,
}

pub fn color(rank: u8, file: u8) -> Color {
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
            let piece = board.search_for((rank, file).into());
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

fn highlighted(board: &Board, selected: &UseSelected) -> Vec<Location> {
    for &selected in &*selected.pos {
        log::info!("Move count {}", moves(board).count());

        if board
            .search_for(selected)
            .is_some()
        {
            return board
                .moves_for_piece(selected)
                .map(|mv| mv.to)
                .collect();
        }
    }
    vec![]
}

fn range(play_as: Color) -> Vec<u8> {
    match play_as {
        Color::White => (0..8).collect(),
        Color::Black => (0..8).rev().collect(),
    }
}
