use std::fmt::Debug;

use super::square::Square;
use crate::board::Board as BoardProps;
use smart_default::SmartDefault;
use yew::prelude::{function_component as component, *};

#[derive(Clone, Properties, PartialEq, Default)]
pub struct Props {
    pub board: BoardProps,
    pub onclick: Callback<(isize, isize)>,
    pub selected: Option<(isize, isize)>,
}

pub fn color(rank: isize, file: isize) -> &'static str {
    if (rank + file) % 2 == 0 {
        "white"
    } else {
        "black"
    }
}

#[component(Board)]
pub fn board(
    Props {
        board,
        onclick,
        selected,
    }: &Props,
) -> Html {
    let mut total = html!();

    for rank in 0..8 {
        let mut row = html!();
        for file in 0..8 {
            let color = color(rank, file);

            let square = html! (
                <Square
                    color={color}
                    file={file}
                    rank={rank}
                    piece={board[(rank, file)]}
                    onclick={onclick}
                    selected={*selected}

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
    return total;
}
