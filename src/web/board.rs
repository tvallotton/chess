use super::square::Square;
use crate::board::Board as BoardProps;
use yew::prelude::{function_component as component, *};

#[derive(Debug, Clone, Copy, Properties, PartialEq)]
pub struct Props {
    pub board: BoardProps,
}

pub fn color(rank: isize, file: isize) -> &'static str {
    if (rank + file) % 2 == 0 {
        "white"
    } else {
        "black"
    }
}

#[component(Board)]
pub fn board(Props { board }: &Props) -> Html {
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
    html!(
        <div>
            <div class="board-row">
                <Square color={"white"} rank={0} file={0} />
                <Square color={"black"} rank={1} file={0} />
                <Square color={"white"} rank={2} file={0} />
                <Square color={"black"} rank={3} file={0} />
                <Square color={"white"} rank={4} file={0} />
                <Square color={"black"} rank={5} file={0} />
                <Square color={"white"} rank={6} file={0} />
                <Square color={"black"} rank={7} file={0} />
            </div>
            <div class="board-row">
                <Square color={"black"} rank={0} file={1} />
                <Square color={"white"} rank={1} file={1} />
                <Square color={"black"} rank={2} file={1} />
                <Square color={"white"} rank={3} file={1} />
                <Square color={"black"} rank={4} file={1} />
                <Square color={"white"} rank={5} file={1} />
                <Square color={"black"} rank={6} file={1} />
                <Square color={"white"} rank={7} file={1} />
            </div>
            <div class="board-row">
                <Square color={"white"} rank={0} file={1} />
                <Square color={"black"} rank={1} file={1} />
                <Square color={"white"} rank={2} file={1} />
                <Square color={"black"} rank={3} file={1} />
                <Square color={"white"} rank={4} file={1} />
                <Square color={"black"} rank={5} file={1} />
                <Square color={"white"} rank={6} file={1} />
                <Square color={"black"} rank={7} file={1} />
            </div>
            <div class="board-row">
                <Square color={"black"} rank={0} file={1} />
                <Square color={"white"} rank={1} file={1} />
                <Square color={"black"} rank={2} file={1} />
                <Square color={"white"} rank={3} file={1} />
                <Square color={"black"} rank={4} file={1} />
                <Square color={"white"} rank={5} file={1} />
                <Square color={"black"} rank={6} file={1} />
                <Square color={"white"} rank={7} file={1} />
            </div>
            <div class="board-row">
                <Square color={"white"} rank={0} file={1} />
                <Square color={"black"} rank={1} file={1} />
                <Square color={"white"} rank={2} file={1} />
                <Square color={"black"} rank={3} file={1} />
                <Square color={"white"} rank={4} file={1} />
                <Square color={"black"} rank={5} file={1} />
                <Square color={"white"} rank={6} file={1} />
                <Square color={"black"} rank={7} file={1} />
            </div>

            <div class="board-row">
                <Square color={"black"} rank={0} file={1} />
                <Square color={"white"} rank={1} file={1} />
                <Square color={"black"} rank={2} file={1} />
                <Square color={"white"} rank={3} file={1} />
                <Square color={"black"} rank={4} file={1} />
                <Square color={"white"} rank={5} file={1} />
                <Square color={"black"} rank={6} file={1} />
                <Square color={"white"} rank={7} file={1} />
            </div>
            <div class="board-row">
                <Square color={"white"} rank={0} file={1} />
                <Square color={"black"} rank={1} file={1} />
                <Square color={"white"} rank={2} file={1} />
                <Square color={"black"} rank={3} file={1} />
                <Square color={"white"} rank={4} file={1} />
                <Square color={"black"} rank={5} file={1} />
                <Square color={"white"} rank={6} file={1} />
                <Square color={"black"} rank={7} file={1} />
            </div>
            <div class="board-row">
                <Square color={"black"} rank={0} file={1} />
                <Square color={"white"} rank={1} file={1} />
                <Square color={"black"} rank={2} file={1} />
                <Square color={"white"} rank={3} file={1} />
                <Square color={"black"} rank={4} file={1} />
                <Square color={"white"} rank={5} file={1} />
                <Square color={"black"} rank={6} file={1} />
                <Square color={"white"} rank={7} file={1} />
            </div>
        </div>

    )
}
