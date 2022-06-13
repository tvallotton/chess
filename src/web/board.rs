use super::square::Square;
use crate::board::Board as BoardProps;
use crate::piece::Color;
use yew::prelude::{function_component as component, *};

#[derive(Clone, Properties, PartialEq, Default)]
pub struct Props {
    pub board: BoardProps,
    pub onclick: Callback<(i8, i8)>,
    pub selected: Option<(i8, i8)>,
    pub play_as: Color,
}

pub fn color(rank: i8, file: i8) -> &'static str {
    if (rank + file) % 2 == 0 {
        "white"
    } else {
        "black"
    }
}

fn highlighted(board: &BoardProps, selected: Option<(i8, i8)>) -> Vec<(i8, i8)> {
    if let Some(pos) = selected {
        board
            .highlighted_squares(pos.into())
            .map(|mv| mv.to)
            .map(|pos| (pos.rank, pos.file))
            .collect()
    } else {
        vec![]
    }
}

#[component(Board)]
pub fn board(
    Props {
        board,
        onclick,
        selected,
        play_as,
        ..
    }: &Props,
) -> Html {
    let mut total = html!();
    let highlighted = &highlighted(board, *selected);
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

            let square = html! (
                <Square
                    color={color}
                    file={file}
                    rank={rank}
                    piece={board[(rank, file)]}
                    onclick={onclick}
                    selected={selected == &Some((rank, file))}
                    highlighted={highlighted.contains(&(rank, file))}

                />
            );
            row = html!(
                <>{row} {square}</>
            );
        }
        total = html!(
            <>
                {total}
                <div class="board-row">
                    {row}
                </div>
            </>
        )
    }
    total
}
