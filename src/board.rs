use dioxus::prelude::*;
use engine::{Board, Color};

use crate::square::Square;

#[derive(Props, PartialEq, Eq)]
pub struct Props<'a> {
    board: Board,
    highlighted: &'a [(i32, i32)],
    play_as: Color, 
}

pub fn color(rank: i8, file: i8) -> Color {
    if (rank + file) % 2 == 0 {
        Color::White
    } else {
        Color::Black
    }
}

pub const Board: Component<Props> = |s| {
    let board = &s.props.board; 
    let mut total = html!();
    let play_as = s.props.play_as; 
    let rank_range: Vec<_> = match play_as {
        Color::White => (0..8).collect(),
        Color::Black => (0..8).rev().collect(),
    };
    let file_range: Vec<_> = match play_as {
        Color::White => (0..8).collect(),
        Color::Black => (0..8).rev().collect(),
    };

    for rank in rank_range {
        let mut row = html!();
        for &file in &file_range {
            let color = color(rank, file);
            let piece = board[(rank, file)]; 
            let square = html! (
                <Square color={color} piece={piece}/>
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
